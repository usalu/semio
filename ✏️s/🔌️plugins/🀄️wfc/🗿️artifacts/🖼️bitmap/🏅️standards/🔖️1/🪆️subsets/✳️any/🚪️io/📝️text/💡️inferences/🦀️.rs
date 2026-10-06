//! 📝️ Inference request and result codecs.

#[allow(unused_imports)]
mod inference_runtime_codec {
use crate::schema::snapshot::{BitmapSnapshot};
use crate::standards::v1::subsets::any::io::text::snapshot::{encode_base64};
use semio_s_plugin_wfc_engine as engine;
use crate::host::inferences::*;
use crate::standards::v1::subsets::any::schema::inferences::*;
impl BitmapInferenceJob {
    /// 📝️ The next slice of commit JSON to append, with the cursor state it leaves behind. Peeked
    /// rather than consumed so a page that cannot take it is committed and the SAME slice is
    /// retried on the next page — the commit text is generated incrementally and never materialised
    /// as one oversized `String`, which is what keeps the guest's contiguous-allocation ceiling out
    /// of the picture for a 256 × 256 output.
    pub(crate) fn next_encode_chunk(&self) -> Option<(String, EncodePhase, usize)> {
        match self.encode_phase {
            EncodePhase::Open => Some((r#"{"pixels":""#.to_string(), EncodePhase::Pixels, 0)),
            EncodePhase::Pixels => {
                if self.encode_cursor >= self.pixels.len() {
                    return Some((String::new(), EncodePhase::Verdict, 0));
                }
                let end = (self.encode_cursor + BITMAP_ENCODE_CHUNK).min(self.pixels.len());
                Some((self.pixels[self.encode_cursor..end].to_string(), EncodePhase::Pixels, end))
            }
            EncodePhase::Verdict => {
                let verdict = if self.contradiction { r#"","contradiction":true,"entropy":["# } else { r#"","contradiction":false,"entropy":["# };
                Some((verdict.to_string(), EncodePhase::Entropy, 0))
            }
            EncodePhase::Entropy => {
                if self.encode_cursor >= self.entropy.len() {
                    return Some((String::new(), EncodePhase::Close, 0));
                }
                let separator = if self.encode_cursor == 0 { "" } else { "," };
                Some((format!("{separator}{}", semio_framework_pack_json::to_json_string(&self.entropy[self.encode_cursor])), EncodePhase::Entropy, self.encode_cursor + 1))
            }
            EncodePhase::Close => None,
        }
    }
}
}
pub(crate) fn decode_inference_value<T:semio_framework_value::FromValue>(text:&str)->Result<T,String> {crate::standards::v1::subsets::any::io::text::bitmap_json_decode(text).map_err(|error|error.to_string())}
pub(crate) fn encode_inference_value<T:semio_framework_value::ToValue>(value:&T)->String {crate::standards::v1::subsets::any::io::text::bitmap_json_encode(value)}
