import sys

def pack_impl(ty, post=None, prefix="store::", pre=""):
    decode_tail = f"        Self::__dsl_from_record(&record).map_err({prefix}text_error_to_pack_error)\n" if post is None else f"        let snapshot = Self::__dsl_from_record(&record).map_err({prefix}text_error_to_pack_error)?;\n{post}"
    return f'''impl store::ArtifactPack for {ty} {{
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {{
{pre}        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }}
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {{
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {{
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {{}}.pack v1, got {{}}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }}
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
{decode_tail}    }}
    fn record_spec() -> Option<dsl::RecordSpec> {{
        Some(Self::__dsl_spec())
    }}
}}
'''

def block_end(s, start):
    depth=0
    i=s.index("{",start)
    while True:
        c=s[i]
        if c=="{": depth+=1
        elif c=="}":
            depth-=1
            if depth==0: return i+1
        i+=1

def replace_impl(s, ty, post=None, pre=""):
    key=f"impl store::ArtifactPack for {ty} {{"
    a=s.index(key)
    b=block_end(s,a)
    if s[b]=="\n": b+=1
    return s[:a]+pack_impl(ty,post,pre=pre)+s[b:]

def cut(s, begin, end, keep_end=False):
    a=s.index(begin); b=s.index(end,a)
    if not keep_end: b+=len(end)
    return s[:a]+s[b:]

VALIDATE_PRE="        self.validate().map_err(store::PackError::Schema)?;\n"
VALIDATE_POST="        snapshot.validate().map_err(store::PackError::Schema)?;\n        Ok(snapshot)\n"
