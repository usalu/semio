use semio_framework_os_flow::artifact::FlowFixture;
use semio_framework_os_kernel::os_store::{ArtifactPack, PackDecodeOptions, PackEncodeOptions};

fn main() {
    let fixture = FlowFixture::default();
    match fixture.encode_pack_with(&PackEncodeOptions::default()) {
        Ok(bytes) => match FlowFixture::decode_pack_with(&bytes, &PackDecodeOptions::default()) {
            Ok(decoded) => println!("[DEBUG] flow pack bytes={} round_trip={}", bytes.len(), decoded == fixture),
            Err(error) => println!("[DEBUG] flow pack decode error={error:?}"),
        },
        Err(error) => println!("[DEBUG] flow pack encode error={error:?}"),
    }
}
