extern crate self as semio_renderer_wgpu_contracts;

#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🤝️contracts/📥️lifted/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod hub_contract;
pub use hub_contract::ShellHubRemoteV1;

#[path = "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️enqueue/🦀️.rs"]
mod enqueue;
#[path = "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎮️input-wire/🦀️.rs"]
mod input_wire;
#[cfg(test)]
#[path = "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🔎️observed-allocator/🦀️.rs"]
mod observed_allocator;
