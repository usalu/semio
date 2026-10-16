#!/usr/bin/env python3
# 📦️ owned_abi moves into abi (plan 6.2); its one reactor-context call stays in the facade: the facade `owned_abi` module re-exports abi's and overrides `deallocate` with the receipt-aware wrapper.
from carvelib import *
import defs

P2 = '#[cfg(all(target_arch = "wasm32", target_env = "p2"))]'
root = read(ROOTRS)
head = P2 + "\n#[doc(hidden)]\npub mod owned_abi {\n"
assert root.count(head) == 1
i = root.index(head)
j = defs.matching_brace(root, i + len(head) - 2) + 1
module = root[i:j]
body = module[len(head):-1]
dealloc_old = '''    pub unsafe fn deallocate(pointer: u32, length: u32) {
        if crate::reactor::original_actor_context::exports::fixed_receipt_pointer(pointer,length)||crate::reactor::original_actor_context::exports::release_owned_job_receipt(pointer,length) { return; }
        let slice = std::ptr::slice_from_raw_parts_mut(pointer as usize as *mut u8, length as usize);
        drop(unsafe { Box::from_raw(slice) });
    }
'''
assert body.count(dealloc_old) == 1
abi_body = body.replace(dealloc_old, '''    pub unsafe fn deallocate(pointer: u32, length: u32) {
        let slice = std::ptr::slice_from_raw_parts_mut(pointer as usize as *mut u8, length as usize);
        drop(unsafe { Box::from_raw(slice) });
    }
''')
F = PLUG + "/🧬️schema/🔁️persistent-local/🦀️.rs"
ft = read(F)
write(F, ft.rstrip("\n") + "\n\n//#region 📦️OwnedAbi\n" + head + abi_body + "}\n//#endregion 📦️OwnedAbi\n")
shim = head + '''    pub use semio_framework_plugin_abi::owned_abi::*;

    /// 📬️ Releases one guest allocation after the reactor-context receipt registries had their chance to claim it; the plain release is abi's.
    pub unsafe fn deallocate(pointer: u32, length: u32) {
        if crate::reactor::original_actor_context::exports::fixed_receipt_pointer(pointer,length)||crate::reactor::original_actor_context::exports::release_owned_job_receipt(pointer,length) { return; }
        unsafe { semio_framework_plugin_abi::owned_abi::deallocate(pointer, length) }
    }
}'''
write(ROOTRS, root[:i] + shim + root[j:])
R = PLUG + "/🚪️io/🦀️.rs"
t = read(R)
old = f"{P2}\n#[doc(hidden)]\npub use persistent_local::component_persistent_local;\n"
assert t.count(old) == 1
write(R, t.replace(old, f"{P2}\n#[doc(hidden)]\npub use persistent_local::{{component_persistent_local, owned_abi}};\n"))
