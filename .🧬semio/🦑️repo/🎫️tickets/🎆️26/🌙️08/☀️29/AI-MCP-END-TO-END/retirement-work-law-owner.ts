import{readFileSync,writeFileSync,renameSync,existsSync,appendFileSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),b="🧰️framework/🔨️modules/🌱️value/♻️retirement/",entry=b+"🎮️controlled/🎟️work/🧪️tests/owning.rs";
function edit(f:string,fn:(s:string)=>string){const s=readFileSync(f,"utf8"),n=fn(s),q=join(t,"🗑️generated/retirement-work-law-"+Date.now()+".rs");writeFileSync(q,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent "+f);renameSync(q,f)}
const f=b+"🎮️controlled/🦀️.rs";edit(f,s=>{const re=/#\[cfg\(test\)\]\s*#\[path\s*=\s*"🎟️work\/🧪️tests\/🦀️\.rs"\]\s*mod \w+;\s*/g;let count=0,n=s.replace(re,()=>{count++;return""});if(count!==1)throw Error("work law declaration "+count);return n});
if(existsSync(entry))throw Error("existing entry");writeFileSync(entry,`//! 🎛️ Public original retirement work laws exercise the canonical owner in a dedicated native binary.
pub use semio_framework_value_retirement::*;
pub use semio_framework_value_retirement::retirement::*;
pub use semio_framework_value_retirement::retirement::controlled::*;
pub use semio_framework_value_factory::{FactoryPayloadRetirement,FactoryRetirement,FactoryAuthority,FactoryChildTickets,FactoryChildSlot,FactoryRetirementTicket,FactoryRetirementAdmissionError,factory_arc_birth_bytes,factory_retirement_frame_bytes,factory_constructor_birth_bytes};
pub use semio_framework_value_factory::owned::FactoryOwnedRetirement;
pub(crate) use semio_framework_value_testing_allocator as allocation_testing;
pub(crate) use std::mem::size_of;
#[path="🦀️.rs"]
mod original_retirement_work_tests;
`);
edit(b+"📦️packages/🦀️rust/Cargo.toml",s=>s+'\n[[test]]\nname = "original-retirement-work"\npath = "../../🎮️controlled/🎟️work/🧪️tests/owning.rs"\n');
appendFileSync(join(t,"native-work-view-2026-11-06.md"),"\nThe actual retirement work/conservation suite now runs through its own explicit native integration target original-retirement-work. Its law body, fixtures and assertions stay in their existing locations and import canonical public retirement/factory/allocator owners through a small owning entry. Removing only the former inline module mount releases the monolithic cfg(test) Source closure without skipping the suite or raising caps. Compilation and the full marker migration remain pending.\n");console.log("Registered all original retirement work assertions as a real native integration target");
