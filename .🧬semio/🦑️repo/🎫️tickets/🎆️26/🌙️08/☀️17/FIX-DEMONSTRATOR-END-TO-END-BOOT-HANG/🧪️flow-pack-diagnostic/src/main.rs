use semio_framework_os_flow::{os_store::ArtifactPack, FlowFixture};

fn main() {
    let fixture = FlowFixture::default();
    let encoded = fixture.encode_pack_with(&Default::default());
    println!("{encoded:?}");
}
