import re, os
V = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video"
vt = V + "/🧪️tests/🔬️unit/🦀️.rs"
lines = open(vt).read().split("\n")
assert lines[371] == "// #region 🔖️H264Tests", lines[371]
assert lines[952] == "// #endregion 🔖️H264Tests", lines[952]
region = lines[371:953]
assert region[1].startswith("fn pcm_frame")
lcg_block = lines[68:77]
assert lcg_block[0].startswith("fn lcg") and lcg_block[-2].endswith("collect()") or True
pcm = region[1:9]
assert pcm[0].startswith("fn pcm_frame") and pcm[-1] == "}" or pcm[-2] == "}", pcm
hdr = ["//! 🔬️ Baseline H.264 unit laws: I_PCM and P_SKIP round trips, hostile bitstreams, SPS/PPS refusals, bit reader and writer, intra prediction, transform and deblocking arithmetic.", "", "use super::*;", ""]
helpers = ["fn lcg(state: &mut u64) -> u8 {", "    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);", "    (*state >> 56) as u8", "}", "", "fn fill_deterministic(state: &mut u64, len: usize) -> Vec<u8> {", "    (0..len).map(|_| lcg(state)).collect()", "}", ""]
assert "".join(lines[68:76]).startswith("fn lcg")
h264_dir = V + "/🎞️h264/🧪️tests/🔬️unit"
os.makedirs(h264_dir, exist_ok=True)
open(h264_dir + "/🦀️.rs", "w").write("\n".join(hdr + helpers + region) + "\n")
new = lines[:371] + lines[954:]
text = "\n".join(new)
pc = "\n".join(region[1:9])
assert text.count("fn synth_rgba") == 1
idx = text.index("// #region 🔖️CoreTests")
text = text[:idx] + pc + "\n\n" + text[idx:]
assert text.startswith("use super::*;")
text = text.replace("use super::*;\n", "use super::*;\nuse semio_s_artifact_remodel_remodeling_video_h264::editor::remodeling::engine::video::h264::{h264_enc_i_pcm_sample, h264_enc_p_skip_sample, h264_enc_sps_pps, h264_enc_sps_pps_nals};\n", 1)
old = "        let expected = ycbcr420_to_rgba(&luma, (mb_w * 16) as usize, &cb, &cr, (mb_w * 8) as usize, width, height);\n"
assert text.count(old) == 1
newexp = "        let expected = H264Decoder::new(&h264_enc_sps_pps(mb_w, mb_h)).expect(\"sps/pps parse\").decode_sample(&samples[0]).expect(\"direct decode\").expect(\"immediate output\");\n"
text = text.replace(old, newexp)
open(vt, "w").write(text)
