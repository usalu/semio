//! 🧪️ Compile the real SVG pane modules with only the raster and shared IO boundaries mocked.
extern crate self as infinite_world;
extern crate self as semio_framework_async;
extern crate self as semio_framework_pixels;
extern crate self as semio_renderer_wgpu_contracts;
extern crate self as semio_renderer_wgpu_scene_state;
extern crate self as semio_renderer_wgpu_host_base;

#[derive(Clone)]
pub struct CancelToken;
impl CancelToken {
    pub fn root_now() -> Self { Self }
    pub fn cancel_now(&self) {}
}
pub struct RasterImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}
pub enum RasterUploadRefusal { Busy }
pub const RASTER_UPLOAD_BYTE_CAPACITY: usize = 16_384;
#[cfg(not(target_arch = "wasm32"))]
pub type RuntimeHostWaker = std::sync::Arc<dyn Fn() + Send + Sync>;
#[cfg(target_arch = "wasm32")]
pub type RuntimeHostWaker = std::rc::Rc<dyn Fn()>;

pub mod world {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WorldAssetRequestToken(pub u64);
    #[derive(Clone, Copy)]
    pub enum WorldAssetRequestKind { Glb }
    #[derive(Debug)]
    pub enum WorldAssetFault { ItemCapacity, UrlCapacity }
    #[derive(Debug)]
    pub struct World3dMeshAsset;
    impl World3dMeshAsset { pub fn close_step(&mut self) -> bool { true } }
}

pub mod asset_authority {
    use crate::world::*;
    use std::{collections::VecDeque, sync::Mutex};
    pub struct Authority {
        sequence: u64,
        queued: VecDeque<Owner>,
    }
    #[derive(Debug)]
    pub struct Owner(WorldAssetRequestToken);
    impl Owner {
        pub fn token(&self) -> WorldAssetRequestToken { self.0 }
        pub fn begin_close(&mut self) {}
    }
    static IO: Mutex<Authority> = Mutex::new(Authority { sequence: 0, queued: VecDeque::new() });
    pub fn renderer_asset_io() -> &'static Mutex<Authority> { &IO }
    impl Authority {
        pub fn cancel_request(&mut self, token: WorldAssetRequestToken) {
            self.queued.retain(|owner| owner.0 != token);
        }
    }
    pub fn reserve_renderer_asset_request(_: WorldAssetRequestKind, url: &str) -> Result<WorldAssetRequestToken, WorldAssetFault> {
        if url.len() > 2_048 { return Err(WorldAssetFault::UrlCapacity) }
        let mut io = IO.lock().unwrap();
        io.sequence += 1;
        let token = WorldAssetRequestToken(io.sequence);
        io.queued.push_back(Owner(token));
        Ok(token)
    }
    pub fn take_next_renderer_asset() -> Option<Owner> { IO.lock().unwrap().queued.pop_front() }
    pub fn return_renderer_asset(_: Owner) -> Result<(), Owner> { Ok(()) }
}

#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/📥️asset/🦀️.rs"]
pub mod asset;

pub mod icon_export {
    use crate::{CancelToken, RasterImage, RuntimeHostWaker, world::World3dMeshAsset};
    pub use crate::asset;
    pub struct IconSvgRejected;
    impl IconSvgRejected { pub fn close_step(&mut self) -> bool { true } }
    pub mod preview {
        use super::*;
        pub struct IconSvgPreview;
        impl IconSvgPreview {
            pub fn new(_: &str, _: World3dMeshAsset, _: CancelToken, _: usize) -> Result<Self, IconSvgRejected> { Ok(Self) }
            pub fn with_waker(self, _: Option<RuntimeHostWaker>) -> Self { self }
            pub fn vector_progress(&self) -> Option<(usize, usize)> { None }
            pub fn waits_for_raster(&self) -> bool { false }
            pub fn advance(&mut self) -> Result<Option<RasterImage>, ()> { Ok(None) }
            pub fn close_step(&mut self) -> bool { true }
        }
    }
}

#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖼️icon-render/🖼️svg-pane/🦀️.rs"]
mod pane;

fn main() {
    use icon_export::asset::{self, IconExportAssetFault};
    use pane::{advance, close_step, observe, retry_deadline, Paint};

    let publish = |_: &str, _: &str, _: u32, _: u32, _: &mut Vec<u8>| panic!("no delivery before a valid GLB");
    let mut now = 0.0;
    assert_eq!(observe("retry", "{}", "/retry.glb"), Paint::Rendering);
    for delay in [0.25, 0.5, 1.0, 2.0] {
        println!("{}", (now * 1_000.0) as u64);
        assert!(advance(now, None, publish));
        let mut owner = asset_authority::take_next_renderer_asset().unwrap();
        let token = owner.token();
        owner.begin_close();
        asset_authority::return_renderer_asset(owner).unwrap();
        asset::reject(token, IconExportAssetFault::Transport { retryable: true });
        assert!(advance(now, None, publish));
        assert_eq!(observe("retry", "{}", "/retry.glb"), Paint::Rendering);
        now += delay;
        assert_eq!(retry_deadline(), Some(now));
        assert!(!advance(now - 0.001, None, publish));
    }
    println!("{}", (now * 1_000.0) as u64);
    assert!(advance(now, None, publish));
    let token = asset_authority::take_next_renderer_asset().unwrap().token();
    asset::reject(token, IconExportAssetFault::Transport { retryable: true });
    assert!(advance(now, None, publish));
    assert_eq!(observe("retry", "{}", "/retry.glb"), Paint::Failed);
    assert!(close_step(Some("retry")));
    assert!(retry_deadline().is_none());

    for fault in [
        IconExportAssetFault::Transport { retryable: false },
        IconExportAssetFault::Rejected("invalid GLB".into()),
    ] {
        observe("invalid", "{}", "/invalid.glb");
        assert!(advance(now, None, publish));
        let token = asset_authority::take_next_renderer_asset().unwrap().token();
        asset::reject(token, fault);
        assert!(advance(now, None, publish));
        assert_eq!(observe("invalid", "{}", "/invalid.glb"), Paint::Failed);
        assert!(retry_deadline().is_none());
        assert!(close_step(Some("invalid")));
    }

    observe("cancelled", "{}", "/cancelled.glb");
    assert!(advance(now, None, publish));
    let token = asset_authority::take_next_renderer_asset().unwrap().token();
    asset::reject(token, IconExportAssetFault::Transport { retryable: true });
    assert!(advance(now, None, publish));
    assert!(retry_deadline().is_some());
    pane::cancel("cancelled");
    assert!(retry_deadline().is_none());
    assert!(advance(now + 10.0, None, publish));
    assert!(!advance(now + 10.0, None, publish));
    assert!(asset_authority::take_next_renderer_asset().is_none());
}
