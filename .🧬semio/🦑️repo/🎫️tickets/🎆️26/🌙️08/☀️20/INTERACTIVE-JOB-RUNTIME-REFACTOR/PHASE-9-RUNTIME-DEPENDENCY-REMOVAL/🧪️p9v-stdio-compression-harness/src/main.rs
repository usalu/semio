use std::{
    io::Write,
    time::{Duration, Instant},
};

use semio_framework_job::{root_cancel_token, Generation, InteractiveJob, OperationId, StepBudget, StepContext, StepOutcome};
use semio_s_plugin_stdio::artifacts::deflate::standards::v_rfc1950::subsets::any::io::{
    deflate_raw, deflate_raw_deterministic_compact_high_search, inflate_raw, zlib_compress_deterministic, zlib_compress_illustrator, zlib_decompress, DeflateEncodeJob, TunedDeflateEncodeJob,
};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::Brep;
use semio_s_plugin_stdio::semantic_fingerprint;

fn drive(mut job: DeflateEncodeJob, fuel: u64) -> Vec<u8> {
    let cancel = root_cancel_token();
    let mut sequence = 0;
    loop {
        let mut context = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, u64::MAX), cancel.clone(), || 0, &mut sequence);
        match job.step(&mut context) {
            StepOutcome::Complete(commit) => return commit.output,
            StepOutcome::Yield | StepOutcome::CheckpointReady(_) => {}
            outcome => panic!("unexpected outcome: {outcome:?}"),
        }
    }
}

fn drive_tuned(mut job: TunedDeflateEncodeJob, fuel: u64) -> Vec<u8> {
    let cancel = root_cancel_token();
    let mut sequence = 0;
    loop {
        let mut context = StepContext::new(OperationId(5), Generation(1), StepBudget::new(fuel, u64::MAX), cancel.clone(), || 0, &mut sequence);
        match job.step(&mut context) {
            StepOutcome::Complete(commit) => return commit.output,
            StepOutcome::Yield => {}
            outcome => panic!("unexpected tuned outcome: {outcome:?}"),
        }
    }
}

fn zip_member<'a>(archive: &'a [u8], wanted: &str) -> &'a [u8] {
    let mut offset = 0;
    while archive.get(offset..offset + 4) == Some(b"PK\x03\x04") {
        let compressed = u32::from_le_bytes(archive[offset + 18..offset + 22].try_into().unwrap()) as usize;
        let name_length = u16::from_le_bytes(archive[offset + 26..offset + 28].try_into().unwrap()) as usize;
        let extra_length = u16::from_le_bytes(archive[offset + 28..offset + 30].try_into().unwrap()) as usize;
        let name_start = offset + 30;
        let payload_start = name_start + name_length + extra_length;
        if std::str::from_utf8(&archive[name_start..name_start + name_length]).unwrap() == wanted {
            return &archive[payload_start..payload_start + compressed];
        }
        offset = payload_start + compressed;
    }
    panic!("missing ZIP member {wanted}")
}

fn main() {
    let fingerprint = semantic_fingerprint(&"stdio-blake3-golden").unwrap();
    let handle = Brep::new().box_prim_sync(1.0, 1.0, 1.0).unwrap();
    assert_eq!(fingerprint, [250, 74, 204, 25, 91, 24, 60, 7, 57, 60, 240, 40, 61, 163, 253, 76, 239, 198, 182, 56, 230, 23, 142, 223, 149, 26, 49, 145, 142, 66, 200, 236]);
    assert_eq!(handle.as_str(), "a63eafbdcde2275214b073904583c60e7888e7c567c3fe41c5051f1bcc21dece");
    println!("[DEBUG] semantic_fingerprint={fingerprint:?}");
    println!("[DEBUG] brep_handle={}", handle.as_str());

    let payload = b"streaming DEFLATE streaming DEFLATE streaming DEFLATE".repeat(64);
    let expected = deflate_raw(&payload);
    for fuel in [1, 2, 7, 64, 1024] {
        assert_eq!(drive(DeflateEncodeJob::new(payload.clone(), 29), fuel), expected);
    }

    let checkpoint_payload = b"checkpointed owned compression ".repeat(256);
    let checkpoint_expected = deflate_raw(&checkpoint_payload);
    let mut job = DeflateEncodeJob::new(checkpoint_payload, 31);
    let mut sequence = 0;
    let checkpoint = loop {
        let mut context = StepContext::new(OperationId(2), Generation(1), StepBudget::new(5, u64::MAX), root_cancel_token(), || 0, &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
            break checkpoint;
        }
    };
    let restored = DeflateEncodeJob::from_checkpoint(&checkpoint.state).unwrap();
    assert_eq!(checkpoint.applied_progress as usize, restored.progress().0);
    assert_eq!(drive(restored, 3), checkpoint_expected);

    let mut cancelled = DeflateEncodeJob::new(vec![7; 4096], 64);
    let before = cancelled.checkpoint_bytes();
    let cancel = root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut context = StepContext::new(OperationId(3), Generation(1), StepBudget::new(1, u64::MAX), cancel, || 0, &mut sequence);
    assert_eq!(cancelled.step(&mut context), StepOutcome::Cancelled);
    assert_eq!(cancelled.checkpoint_bytes(), before);

    let mut adversarial = Vec::with_capacity(256 * 1024);
    for index in 0..256 * 1024 {
        adversarial.push(((index * 31) ^ (index >> 5)) as u8);
    }
    let mut timed = DeflateEncodeJob::new(adversarial.clone(), usize::MAX);
    let mut sequence = 0;
    let mut maximum = Duration::ZERO;
    loop {
        let mut context = StepContext::new(OperationId(4), Generation(1), StepBudget::new(64, u64::MAX), root_cancel_token(), || 0, &mut sequence);
        let started = Instant::now();
        let outcome = timed.step(&mut context);
        maximum = maximum.max(started.elapsed());
        if matches!(outcome, StepOutcome::Complete(_)) {
            break;
        }
    }
    assert!(maximum < Duration::from_millis(8), "maximum step {maximum:?}");

    let mut timed_tuned = TunedDeflateEncodeJob::level_nine(adversarial.clone());
    let tuned_cancel = root_cancel_token();
    let mut sequence = 0;
    let mut maximum_tuned = Duration::ZERO;
    let tuned_output = loop {
        let mut context = StepContext::new(OperationId(7), Generation(1), StepBudget::new(1, u64::MAX), tuned_cancel.clone(), || 0, &mut sequence);
        let started = Instant::now();
        let outcome = timed_tuned.step(&mut context);
        maximum_tuned = maximum_tuned.max(started.elapsed());
        if let StepOutcome::Complete(commit) = outcome {
            break commit.output;
        }
    };
    assert!(maximum_tuned < Duration::from_millis(8), "maximum tuned step {maximum_tuned:?}");
    assert_eq!(zlib_decompress(&tuned_output).unwrap(), adversarial);

    let fixture = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../temp/domai-specific-programmaning-language-for-architects.pptx")).unwrap();
    for path in ["ppt/embeddings/oleObject1.bin", "ppt/embeddings/oleObject2.bin", "ppt/embeddings/oleObject3.bin"] {
        let golden = zip_member(&fixture, path);
        let decoded = inflate_raw(golden).unwrap();
        assert_eq!(deflate_raw_deterministic_compact_high_search(&decoded).unwrap(), golden, "{path}");
    }

    let pdf = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../temp/📄️bachelor-thesis.pdf")).unwrap();
    let marker = b"/Length 3362\n/Filter /FlateDecode\n>>\nstream\n";
    let start = pdf.windows(marker.len()).position(|window| window == marker).unwrap() + marker.len();
    let illustrator_golden = &pdf[start..start + 3362];
    let illustrator_payload = zlib_decompress(illustrator_golden).unwrap();
    assert_eq!(zlib_compress_illustrator(&illustrator_payload).unwrap(), illustrator_golden);
    let mut reference = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::new(9));
    reference.write_all(&illustrator_payload).unwrap();
    assert_eq!(zlib_compress_deterministic(&illustrator_payload).unwrap(), reference.finish().unwrap());

    let mut incompressible = Vec::with_capacity(64 * 1024);
    let mut state = 0x9e37_79b9u32;
    for _ in 0..64 * 1024 {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        incompressible.push(state as u8);
    }
    for payload in [Vec::new(), (0..=255).collect(), incompressible] {
        let encoded = zlib_compress_deterministic(&payload).unwrap();
        assert_eq!(zlib_decompress(&encoded).unwrap(), payload);
        for fuel in [1, 7, 64, 1024] {
            assert_eq!(drive_tuned(TunedDeflateEncodeJob::level_nine(payload.clone()), fuel), encoded);
        }
    }

    for fuel in [1, 7, 64, 1024] {
        assert_eq!(drive_tuned(TunedDeflateEncodeJob::illustrator(illustrator_payload.clone()), fuel), illustrator_golden);
    }

    let tuned_checkpoint_payload = b"checkpointed tuned compression ".repeat(256);
    let tuned_checkpoint_expected = zlib_compress_deterministic(&tuned_checkpoint_payload).unwrap();
    let mut checkpointed = TunedDeflateEncodeJob::level_nine(tuned_checkpoint_payload);
    let cancel = root_cancel_token();
    let mut sequence = 0;
    for _ in 0..127 {
        let mut context = StepContext::new(OperationId(8), Generation(1), StepBudget::new(1, u64::MAX), cancel.clone(), || 0, &mut sequence);
        assert_eq!(checkpointed.step(&mut context), StepOutcome::Yield);
    }
    let restored = TunedDeflateEncodeJob::from_checkpoint(&checkpointed.checkpoint_bytes()).unwrap();
    assert_eq!(drive_tuned(restored, 3), tuned_checkpoint_expected);

    let mut cancelled = TunedDeflateEncodeJob::level_nine(vec![3; 4096]);
    let before = cancelled.progress();
    let cancel = root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut context = StepContext::new(OperationId(6), Generation(1), StepBudget::new(1, u64::MAX), cancel, || 0, &mut sequence);
    assert_eq!(cancelled.step(&mut context), StepOutcome::Cancelled);
    assert_eq!(cancelled.progress(), before);

    let golden = deflate_raw(b"stdio-deflate-stream-golden-stdio-deflate-stream-golden");
    println!("[DEBUG] golden={golden:?}");
    println!("[DEBUG] maximum_step_us={}", maximum.as_micros());
    println!("[DEBUG] maximum_tuned_step_us={}", maximum_tuned.as_micros());
    println!("[DEBUG] compression_harness=pass");
}
