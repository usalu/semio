"""🧹️ Pass-2 leftovers in my families that the ledger cannot see: apply-rejection closures whose parameter only fed the
dropped prose (`|error|` → `|_|`), `MutationApplyError` struct literals that still name a `mutation.*` string and a message
(forms, semio object → `MutationApplyError::new(MutationCode::Invariant)`), and semio's `Rejected` diff codec, which
serialized the removed message field (now the rejection's own JSON wire form, hex-armoured as before)."""
import re

CLOSURE_FILES = [
    "🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs",
    "🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs",
    "🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs",
    "🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs",
    "🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs",
    "🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs",
]
CLOSURE = re.compile(r"\|(\w+)\|(\s*(?:protocol::)?MutationApplyError::new\((?:protocol::)?MutationCode::\w+\)(?:\.at\(\[[^\]]*\]\))?)(?=\))")


def unused_closure_parameters(text: str) -> str:
    """🧹️ A rejection closure that no longer reads its parameter takes `_`."""
    return CLOSURE.sub(lambda match: match.group(0) if re.search(r"\b" + match.group(1) + r"\b", match.group(2)) else "|_|" + match.group(2), text)


FORMS_DIFF = "📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"
OBJECT_DIFF = "🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🔺️diff/🦀️.rs"
SEMIO_BASE_DIFF = "🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🦀️.rs"
CHILD_REFUSAL = 'self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.child-identity".into(), message, target: Vec::new() })?;'

EDITS = [
    (FORMS_DIFF, CHILD_REFUSAL, "self.validate().map_err(|_| protocol::MutationApplyError::new(protocol::MutationCode::Invariant))?;"),
    (OBJECT_DIFF, CHILD_REFUSAL, "self.validate().map_err(|_| protocol::MutationApplyError::new(protocol::MutationCode::Invariant))?;"),
    (SEMIO_BASE_DIFF, '''fn enc_rejection(error: &MutationApplyError) -> String {
    std::iter::once(error.code.as_str())
        .chain(std::iter::once(error.message.as_str()))
        .chain(error.target.iter().map(String::as_str))
        .map(|value| value.as_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
}''', '''fn enc_rejection(error: &MutationApplyError) -> String {
    dsl::json::to_json_string(error).as_bytes().iter().map(|byte| format!("{byte:02x}")).collect()
}'''),
    (SEMIO_BASE_DIFF, '''fn dec_rejection(payload: &str) -> Result<MutationApplyError, String> {
    let fields = payload
        .split(',')
        .map(|hex| {
            if hex.len() % 2 != 0 {
                return Err("rejected: odd hex length".to_string());
            }
            let bytes = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16)).collect::<Result<Vec<_>, _>>().map_err(|error| format!("rejected: invalid hex: {error}"))?;
            String::from_utf8(bytes).map_err(|error| format!("rejected: utf8 decode: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if fields.len() < 2 {
        return Err("rejected: expected code and message".to_string());
    }
    Ok(MutationApplyError { code: fields[0].clone(), message: fields[1].clone(), target: fields[2..].to_vec() })
}''', '''fn dec_rejection(payload: &str) -> Result<MutationApplyError, String> {
    if payload.len() % 2 != 0 {
        return Err("rejected: odd hex length".to_string());
    }
    let bytes = (0..payload.len()).step_by(2).map(|index| u8::from_str_radix(&payload[index..index + 2], 16)).collect::<Result<Vec<_>, _>>().map_err(|error| format!("rejected: invalid hex: {error}"))?;
    let text = String::from_utf8(bytes).map_err(|error| format!("rejected: utf8 decode: {error}"))?;
    dsl::json::from_json_str(&text).map_err(|error| format!("rejected: json decode: {error}"))
}'''),
]

TRANSFORMS = [(path, unused_closure_parameters) for path in CLOSURE_FILES]
