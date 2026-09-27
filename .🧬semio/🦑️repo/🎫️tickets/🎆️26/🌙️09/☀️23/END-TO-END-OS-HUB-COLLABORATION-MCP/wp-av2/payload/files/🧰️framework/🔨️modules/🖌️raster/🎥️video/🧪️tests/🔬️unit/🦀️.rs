use super::*;

/// 🧫️ The language-agnostic fixture both twins answer (`🟦️.ts` beside `🦀️.rs` reads the same file).
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("raster video fixture")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn parameters(case: &serde_json::Value) -> VideoStreamParameters {
    let number = |key: &str| case[key].as_u64().expect("fixture integer") as u32;
    VideoStreamParameters { width: number("width"), height: number("height"), fps: number("fps") }
}

fn solid(parameters: &VideoStreamParameters, rgb: &serde_json::Value) -> Vec<u8> {
    let channel = |index: usize| rgb[index].as_u64().expect("rgb channel") as u8;
    [channel(0), channel(1), channel(2), 255].repeat(parameters.width as usize * parameters.height as usize)
}

struct Recording<'a> {
    inner: &'a mut AvcIntraPcmEncoder,
    sizes: Vec<u64>,
}

impl VideoEncoder for Recording<'_> {
    fn parameters(&self) -> VideoStreamParameters {
        self.inner.parameters()
    }
    fn configuration(&self) -> &AvcDecoderConfiguration {
        self.inner.configuration()
    }
    fn encode(&mut self, rgba: &[u8]) -> Result<EncodedVideoSample, VideoEncodeError> {
        let sample = self.inner.encode(rgba)?;
        self.sizes.push(sample.data.len() as u64);
        Ok(sample)
    }
}

/// ⚖️ LAW: every fixture case encodes to exactly the pinned SPS/PPS, sample sizes, container size and
/// (for the small cases) container bytes — the bytes ffprobe/ffmpeg accepted as Constrained Baseline
/// H.264 with the declared size, frame count, duration, level and colours (`fixture.oracle`).
#[test]
fn every_fixture_case_encodes_to_the_pinned_stream() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let expected = &case["expected"];
        let parameters = parameters(case);
        let mut encoder = AvcIntraPcmEncoder::new(parameters).expect("admitted stream");
        let pictures: Vec<(Vec<u8>, u32)> = case["runs"].as_array().expect("runs").iter().map(|run| (solid(&parameters, &run["rgb"]), run["frames"].as_u64().expect("frames") as u32)).collect();
        let runs: Vec<VideoFrameRun<'_>> = pictures.iter().map(|(rgba, frames)| VideoFrameRun { rgba, frames: *frames }).collect();
        let mut reported = Vec::new();
        let mut recording = Recording { inner: &mut encoder, sizes: Vec::new() };
        let mp4 = encode_video_runs(&mut recording, &runs, &mut |done, total| reported.push((done, total)), &|| false).expect("encoded");
        let sizes = recording.sizes.clone();
        let frames = sizes.len() as u32;
        assert_eq!(u64::from(frames), expected["frameCount"].as_u64().expect("frameCount"), "{id}: frame count");
        assert_eq!(reported.last().copied(), Some((frames, frames)), "{id}: progress ends at total");
        assert_eq!(u64::from(video_duration_milliseconds(frames, parameters.fps)), expected["durationMs"].as_u64().expect("durationMs"), "{id}: duration");
        assert_eq!(u64::from(parameters.macroblock_width()), expected["macroblocks"]["width"].as_u64().expect("mb width"), "{id}: macroblock width");
        assert_eq!(u64::from(parameters.macroblock_height()), expected["macroblocks"]["height"].as_u64().expect("mb height"), "{id}: macroblock height");
        assert_eq!(u64::from(parameters.level_idc()), expected["levelIdc"].as_u64().expect("levelIdc"), "{id}: level");
        assert_eq!(avc_codec_string(encoder.configuration()), expected["codec"].as_str().expect("codec"), "{id}: codec string");
        assert_eq!(hex(&encoder.configuration().sps), expected["spsHex"].as_str().expect("sps"), "{id}: SPS bytes");
        assert_eq!(hex(&encoder.configuration().pps), expected["ppsHex"].as_str().expect("pps"), "{id}: PPS bytes");
        assert_eq!(sizes, expected["sampleSizes"].as_array().expect("sizes").iter().map(|size| size.as_u64().expect("size")).collect::<Vec<_>>(), "{id}: sample sizes");
        assert_eq!(mp4.len() as u64, expected["mp4Bytes"].as_u64().expect("mp4Bytes"), "{id}: container size");
        if let Some(bytes) = expected["mp4Hex"].as_str() {
            assert_eq!(hex(&mp4), bytes, "{id}: container bytes");
        }
        for (run, yuv) in case["runs"].as_array().expect("runs").iter().zip(expected["yuv"].as_array().expect("yuv")) {
            let channel = |index: usize| run["rgb"][index].as_i64().expect("rgb") as i32;
            let (r, g, b) = (channel(0), channel(1), channel(2));
            assert_eq!([bt601_luma(r, g, b), bt601_cb(r, g, b), bt601_cr(r, g, b)].map(i64::from).to_vec(), yuv.as_array().expect("yuv row").iter().map(|value| value.as_i64().expect("yuv")).collect::<Vec<_>>(), "{id}: BT.601 samples");
        }
    }
}

/// ⚖️ LAW: the tier refuses every stream no 4:2:0 H.264 picture of it can carry, with the fixture's error.
#[test]
fn refused_streams_answer_the_declared_error() {
    let fixture = fixture();
    for case in fixture["refusals"].as_array().expect("refusals") {
        let error = AvcIntraPcmEncoder::new(parameters(case)).err().expect("refused stream");
        let code = match error {
            VideoEncodeError::Dimensions { .. } => "dimensions",
            VideoEncodeError::FrameRate { .. } => "frameRate",
            _ => "other",
        };
        assert_eq!(code, case["error"].as_str().expect("error"), "{}", case["id"]);
    }
}

/// ⚖️ LAW: cancellation stops before the next frame and no container is written; a frame of the wrong
/// size is refused rather than read out of bounds.
#[test]
fn cancellation_and_frame_size_are_enforced() {
    let parameters = VideoStreamParameters { width: 32, height: 16, fps: 30 };
    let mut encoder = AvcIntraPcmEncoder::new(parameters).expect("admitted stream");
    let rgba = [0u8, 0, 0, 255].repeat(32 * 16);
    let runs = [VideoFrameRun { rgba: &rgba, frames: 3 }];
    let done = std::cell::Cell::new(0);
    let result = encode_video_runs(&mut encoder, &runs, &mut |frames, _| done.set(frames), &|| done.get() >= 1);
    assert_eq!(result, Err(VideoEncodeError::Cancelled));
    assert_eq!(done.get(), 1);
    assert_eq!(encoder.encode(&rgba[4..]), Err(VideoEncodeError::FrameBytes { expected: rgba.len(), actual: rgba.len() - 4 }));
    assert_eq!(write_avc_mp4(&parameters, encoder.configuration(), &[]), Err(VideoEncodeError::Empty));
}
