# Current five owned UTF8 pinned source

The five staged arbitrary-payload cuts have exact current bodies, hashes, full inverses and original Display expressions; independent Rust grammar reports no errors. Three envelope payloads move their inner Vec; the two Binary conversions clone the caller bytes, preserving the original. No rejected payload recovery is introduced.

The actual configured nightly string.rs full body matches its retained frame. Its String::from_utf8 delegates to str::from_utf8, and FromUtf8Error Display delegates directly to its stored Utf8Error. Both selected source slices match exactly. The staged actualCompilerBinding field is false; an explicit Native compiler/toolchain receipt is still needed for that narrower provenance claim. This is source-only evidence, not actual caller runtime acceptance.

Full proof: [pinned source](🗑️generated/sole-pack-error/independent-five-owned-utf8-pinned-system-source-1.json).
