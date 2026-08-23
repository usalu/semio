use std::io::Cursor;

#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}
impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value),
            _ => None,
        }
    }
    fn str(&self, key: &str) -> String {
        match self.get(key) {
            Some(Json::String(value)) => value.clone(),
            _ => String::new(),
        }
    }
}
fn digest(input: &[u8]) -> String {
    // trivial FNV-1a stand-in, good enough for a smoke test
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", h)
}

fn number(value: &Json, key: &str, fallback: f64) -> f64 {
    match value.get(key) {
        Some(Json::Number(found)) => *found,
        _ => fallback,
    }
}
fn boolean(value: &Json, key: &str, fallback: bool) -> bool {
    match value.get(key) {
        Some(Json::Bool(found)) => *found,
        _ => fallback,
    }
}
fn strings(value: &Json, key: &str) -> Vec<String> {
    match value.get(key) {
        Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::String(text) = item { Some(text.clone()) } else { None }).collect(),
        _ => Vec::new(),
    }
}
fn bytes(value: &Json, key: &str) -> Vec<u8> {
    match value.get(key) {
        Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(n) = item { Some(*n as u8) } else { None }).collect(),
        _ => Vec::new(),
    }
}

struct DecodedMovie {
    major_brand: String,
    minor_version: u32,
    compatible_brands: Vec<String>,
    timescale: u32,
    tracks: Vec<DecodedTrack>,
}
struct DecodedTrack {
    width: u16,
    height: u16,
    timescale: u32,
    sps: Vec<u8>,
    pps: Vec<u8>,
    samples: Vec<::mp4::Mp4Sample>,
}

fn fourcc(value: &str) -> ::mp4::FourCC {
    let mut padded = [b' '; 4];
    for (index, byte) in value.as_bytes().iter().take(4).enumerate() {
        padded[index] = *byte;
    }
    std::str::from_utf8(&padded).unwrap().parse::<::mp4::FourCC>().unwrap()
}

fn read_movie(input: &[u8]) -> Result<DecodedMovie, String> {
    let mut reader = ::mp4::Mp4Reader::read_header(Cursor::new(input.to_vec()), input.len() as u64).map_err(|e| format!("parse: {e}"))?;
    let major_brand = reader.major_brand().to_string();
    let minor_version = reader.minor_version();
    let compatible_brands = reader.compatible_brands().iter().map(|b| b.to_string()).collect();
    let timescale = reader.timescale();
    let mut track_ids: Vec<u32> = reader.tracks().keys().copied().collect();
    track_ids.sort_unstable();
    let mut tracks = Vec::with_capacity(track_ids.len());
    for track_id in track_ids {
        let (width, height, track_timescale, sps, pps) = {
            let track = reader.tracks().get(&track_id).ok_or("vanished")?;
            let sps = track.sequence_parameter_set().map_err(|e| format!("sps: {e}"))?.to_vec();
            let pps = track.picture_parameter_set().map_err(|e| format!("pps: {e}"))?.to_vec();
            (track.width(), track.height(), track.timescale(), sps, pps)
        };
        let count = reader.sample_count(track_id).map_err(|e| format!("count: {e}"))?;
        let mut samples = Vec::with_capacity(count as usize);
        for sample_id in 1..=count {
            if let Some(sample) = reader.read_sample(track_id, sample_id).map_err(|e| format!("sample: {e}"))? {
                samples.push(sample);
            }
        }
        tracks.push(DecodedTrack { width, height, timescale: track_timescale, sps, pps, samples });
    }
    Ok(DecodedMovie { major_brand, minor_version, compatible_brands, timescale, tracks })
}

fn write_movie(movie: &DecodedMovie) -> Result<Vec<u8>, String> {
    let config = ::mp4::Mp4Config { major_brand: fourcc(&movie.major_brand), minor_version: movie.minor_version, compatible_brands: movie.compatible_brands.iter().map(|b| fourcc(b)).collect(), timescale: movie.timescale };
    let mut buffer = Cursor::new(Vec::<u8>::new());
    let mut writer = ::mp4::Mp4Writer::write_start(&mut buffer, &config).map_err(|e| format!("start: {e}"))?;
    for track in &movie.tracks {
        let track_config = ::mp4::TrackConfig {
            track_type: ::mp4::TrackType::Video,
            timescale: track.timescale,
            language: "und".to_string(),
            media_conf: ::mp4::MediaConfig::AvcConfig(::mp4::AvcConfig { width: track.width, height: track.height, seq_param_set: track.sps.clone(), pic_param_set: track.pps.clone() }),
        };
        writer.add_track(&track_config).map_err(|e| format!("add_track: {e}"))?;
    }
    for (index, track) in movie.tracks.iter().enumerate() {
        let track_id = index as u32 + 1;
        for sample in &track.samples {
            writer.write_sample(track_id, sample).map_err(|e| format!("write_sample: {e}"))?;
        }
    }
    writer.write_end().map_err(|e| format!("end: {e}"))?;
    Ok(buffer.into_inner())
}

fn clone_sample(s: &::mp4::Mp4Sample) -> ::mp4::Mp4Sample {
    ::mp4::Mp4Sample { start_time: s.start_time, duration: s.duration, rendering_offset: s.rendering_offset, is_sync: s.is_sync, bytes: s.bytes.clone() }
}

fn owned_sample(value: &Json, key: &str) -> ::mp4::Mp4Sample {
    let entry = value.get(key).cloned().unwrap_or(Json::Object(Vec::new()));
    ::mp4::Mp4Sample { start_time: 0, duration: number(&entry, "duration", 0.0) as u32, rendering_offset: number(&entry, "ctsOffset", 0.0) as i32, is_sync: boolean(&entry, "sync", true), bytes: ::mp4::Bytes::from(bytes(&entry, "data")) }
}

fn project_summary(bytes: &[u8]) -> Result<String, String> {
    let movie = read_movie(bytes)?;
    let mut out = format!("brand={} minor={} brands={:?} tracks={}", movie.major_brand, movie.minor_version, movie.compatible_brands, movie.tracks.len());
    for (i, t) in movie.tracks.iter().enumerate() {
        out += &format!(" | track{i}: {}x{} ts={} spsDigest={} ppsDigest={} samples={}", t.width, t.height, t.timescale, digest(&t.sps), digest(&t.pps), t.samples.len());
        if let Some(first) = t.samples.first() {
            out += &format!(" firstSample(dur={},cts={},sync={},len={})", first.duration, first.rendering_offset, first.is_sync, first.bytes.len());
        }
    }
    Ok(out)
}

fn obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

fn main() {
    let path = std::env::args().nth(1).expect("path");
    let input = std::fs::read(&path).unwrap();
    println!("INPUT: {}", project_summary(&input).unwrap());

    // no-mutation
    println!("\n[no-mutation] (echo)");

    // set-ftyp
    {
        let mut movie = read_movie(&input).unwrap();
        movie.major_brand = "mp42".to_string();
        movie.minor_version = 1;
        movie.compatible_brands = vec!["mp42".to_string(), "isom".to_string()];
        let out = write_movie(&movie).unwrap();
        println!("\n[set-ftyp] out_len={} {}", out.len(), project_summary(&out).unwrap());
    }

    // insert-track
    {
        let mut movie = read_movie(&input).unwrap();
        let source = movie.tracks.first().unwrap();
        let clone = DecodedTrack { width: source.width, height: source.height, timescale: source.timescale, sps: source.sps.clone(), pps: source.pps.clone(), samples: source.samples.iter().map(clone_sample).collect() };
        movie.tracks.insert(1, clone);
        let out = write_movie(&movie).unwrap();
        println!("\n[insert-track] out_len={} {}", out.len(), project_summary(&out).unwrap());
    }

    // remove-track
    {
        let mut movie = read_movie(&input).unwrap();
        movie.tracks.remove(0);
        match write_movie(&movie) {
            Ok(out) => {
                println!("\n[remove-track] out_len={}", out.len());
                match project_summary(&out) {
                    Ok(s) => println!("  re-read ok: {}", s),
                    Err(e) => println!("  re-read FAILED (expected? zero tracks): {}", e),
                }
            }
            Err(e) => println!("\n[remove-track] write FAILED: {}", e),
        }
    }

    // set-track-dimensions
    {
        let mut movie = read_movie(&input).unwrap();
        movie.tracks[0].width = 640;
        movie.tracks[0].height = 480;
        let out = write_movie(&movie).unwrap();
        println!("\n[set-track-dimensions] out_len={} {}", out.len(), project_summary(&out).unwrap());
    }

    // set-track-codec
    {
        let mut movie = read_movie(&input).unwrap();
        movie.tracks[0].sps = vec![0x67, 0x42, 0x00, 0x1e, 0x8c, 0x8d, 0x40];
        movie.tracks[0].pps = vec![0x68, 0xce, 0x3c, 0x80];
        let out = write_movie(&movie).unwrap();
        println!("\n[set-track-codec] out_len={} {}", out.len(), project_summary(&out).unwrap());
    }

    // insert-sample
    {
        let mut movie = read_movie(&input).unwrap();
        let params = obj(vec![("sample", obj(vec![("data", Json::Array(vec![Json::Number(0.0), Json::Number(0.0), Json::Number(0.0), Json::Number(4.0), Json::Number(101.0), Json::Number(1.0), Json::Number(2.0), Json::Number(3.0)])), ("duration", Json::Number(512.0)), ("ctsOffset", Json::Number(0.0)), ("sync", Json::Bool(false))]))]);
        let sample = owned_sample(&params, "sample");
        movie.tracks[0].samples.insert(3, sample);
        let out = write_movie(&movie).unwrap();
        println!("\n[insert-sample] out_len={} {}", out.len(), project_summary(&out).unwrap());
    }

    // remove-sample
    {
        let mut movie = read_movie(&input).unwrap();
        movie.tracks[0].samples.remove(5);
        let out = write_movie(&movie).unwrap();
        println!("\n[remove-sample] out_len={} {}", out.len(), project_summary(&out).unwrap());
    }

    // set-sample-sync
    {
        let mut movie = read_movie(&input).unwrap();
        movie.tracks[0].samples[2].is_sync = !movie.tracks[0].samples[2].is_sync;
        let out = write_movie(&movie).unwrap();
        println!("\n[set-sample-sync] out_len={} {}", out.len(), project_summary(&out).unwrap());
    }

    // set-snapshot (ftyp + drop last sample)
    {
        let mut movie = read_movie(&input).unwrap();
        movie.major_brand = "isom".to_string();
        movie.minor_version = 42;
        if let Some(t) = movie.tracks.first_mut() {
            t.samples.pop();
        }
        let out = write_movie(&movie).unwrap();
        println!("\n[set-snapshot] out_len={} {}", out.len(), project_summary(&out).unwrap());
    }

    println!("\nALL OK");
}
