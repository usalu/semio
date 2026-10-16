#!/usr/bin/env python3
# 🧠️ Kernel group carve, apply step: writes the lifted files, trims the in-place files and builds the four crate roots.
import json, os, re, subprocess, sys
from common import *
from carve import *
import kernel1, needs, provide, items
from kernel1 import K, FILES, PROT, POOLC

def rebase(text, old_dir, new_dir):
    def sub(m):
        lit = m.group(2)
        if os.path.isabs(lit):
            return m.group(0)
        return m.group(1) + os.path.relpath(os.path.normpath(os.path.join(old_dir, lit)), new_dir) + '"'
    text = re.sub(r'(#\[\s*path\s*=\s*")([^"]+)"', sub, text)
    text = re.sub(r'(include!\(\s*")([^"]+)"', sub, text)
    return text

def widen_all(text):
    return text.replace("pub(super)", "pub").replace("pub(crate)", "pub")

def hdr(doc):
    return f"//! {doc}\n\nuse super::*;\n\n"

parent_text = read(FILES["parent"])
parent_doc = parent_text[:parent_text.index("use semio_framework::kernel")]
parent_entries = [e for e in __import__("rs").uses(parent_text) if e[1] == "" and not e[4]]

out = kernel1.main(False)
rep_dir = os.path.dirname(FILES["replay"])
pool_dir = os.path.dirname(FILES["pool"])
ret_dir = os.path.dirname(FILES["retained"])

L1 = PROT + "/📥️lifted/📬️kernel-replay-client/🦀️.rs"
L2 = PROT + "/📥️lifted/🏊️kernel-pool/🦀️.rs"
L3 = PROT + "/📥️lifted/📦️kernel-runtime/🦀️.rs"
L4 = POOLC + "/📥️lifted/🗃️kernel-retained-state/🦀️.rs"

head = rebase(out["head_body"], rep_dir, os.path.dirname(L1))
write(L1, widen_all(head))
write(L2, widen_all(hdr("🏊️ The bounded kernel request queue the pool drains and every client enqueues into.\n//!\n//! 🧭️ Lifted out of the kernel pool into kernel-protocol: the queue is protocol, the pool is one consumer.") + out["queue_text"]))
write(L3, widen_all(hdr("📦️ Reads one completed component into the compiler's contiguous input page by page.\n//!\n//! 🧭️ Lifted out of the kernel runtime root into kernel-protocol.") + out["rnc_text"]))
write(L4, widen_all(hdr("🗃️ The kernel pool state: the thirty fields the pool implementation owns.\n//!\n//! 🧭️ Lifted out of the retained state into kernel-pool, so the struct shares a crate with its inherent implementation.") + out["kps_text"]))
write(FILES["replay"], widen_all(out["client_text"]))
write(FILES["retained"], widen_all(out["rt_rest"]))
pool_rest = out["pool_rest"]
sem = re.search(r'\n#\[cfg\(test\)\]\ninclude!\("[^"]*kernel-runtime-semantic-document/🦀️.rs"\);\n', pool_rest)
assert sem, "semantic document include not found"
sem_include = sem.group(0)
pool_rest = pool_rest.replace(sem_include, "\n")
write(FILES["pool"], widen_all(pool_rest))
write(FILES["core"], widen_all(read(FILES["core"])))
write(T + "/🎠️runtime/🦀️.rs", widen_all(read(T + "/🎠️runtime/🦀️.rs")))
write(R + "/📊️scale-bench/🦀️.rs", widen_all(read(R + "/📊️scale-bench/🦀️.rs")))
for f in (FILES["core"], FILES["replay"], FILES["retained"], FILES["pool"], L1, L2, L3, L4, R + "/📊️scale-bench/🦀️.rs", T + "/🎠️runtime/🦀️.rs"):
    t = read(f)
    n = t.count("crate::program_bridge::ProgramFault")
    t = t.replace("crate::program_bridge::ProgramFault", "ProgramFault")
    write(f, t)
print("kernel files written")
sem_path = re.search(r'include!\("([^"]+)"\)', sem_include).group(1)
SEM_ABS = os.path.normpath(os.path.join(pool_dir, sem_path))
json.dump({"parent_doc": parent_doc, "sem": SEM_ABS}, open(RS + "/kernel2.json", "w"), ensure_ascii=False)
