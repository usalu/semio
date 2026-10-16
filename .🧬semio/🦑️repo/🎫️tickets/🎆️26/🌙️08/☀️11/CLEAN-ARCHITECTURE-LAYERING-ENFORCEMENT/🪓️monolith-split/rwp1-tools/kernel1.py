#!/usr/bin/env python3
# 🧠️ Kernel group carve (kernel-protocol, kernel-retained, kernel-pool, kernel-runtime): cuts the item groups of plan 5.2 into lifted files.
import re, sys, os
from common import *
from carve import *
import items

K = R + "/📦️kernel-runtime"
FILES = dict(core=K + "/🧮️kernel-core/🦀️.rs", replay=K + "/📬️kernel-replay-client/🦀️.rs", retained=K + "/🗃️kernel-retained-state/🦀️.rs", pool=K + "/🏊️kernel-pool/🦀️.rs", parent=K + "/🦀️.rs")
PROT = T + "/" + CRATES["kernel-protocol"]
POOLC = T + "/" + CRATES["kernel-pool"]


def open_regions(lines, upto):
    stack = []
    for l in lines[:upto]:
        m = re.match(r"\s*//#region (.*)", l)
        if m:
            stack.append(m.group(1))
        m = re.match(r"\s*//#endregion\b", l)
        if m and stack:
            stack.pop()
    return stack


def cut(text, start, end):
    """1-based inclusive line range; returns (remaining, removed_with_balanced_regions)."""
    L = text.split("\n")
    stack = open_regions(L, start - 1)
    removed = L[start - 1:end]
    inner = open_regions(removed, len(removed))
    pre = [f"//#region {r}" for r in stack]
    post = [f"//#endregion {r}" for r in reversed(stack)]
    # regions opened inside the removed text and not closed there stay open in the remainder: keep their markers unbalanced in the moved text (caller checks)
    rest = L[:start - 1] + L[end:]
    return "\n".join(rest), "\n".join(pre + removed + post) + "\n", inner


def lines_of(text, name, kind=None, prefix=None):
    out = [it for it in items.table(text) if (it["name"] == name or (prefix and it["name"].startswith(prefix))) and (kind is None or it["kind"] == kind)]
    return out


def main(apply):
    texts = {k: read(v) for k, v in FILES.items()}
    # 1. replay head vs client
    kf = [it for it in items.table(texts["replay"]) if it["name"] == "KernelFuture" and it["kind"] == "struct"][0]
    split = kf["start"]
    L = texts["replay"].split("\n")
    head_text = "\n".join(L[:split - 1])
    stack = open_regions(L, split - 1)
    print("replay split at", split, "open regions", stack)
    client_text = "//! 📬️ Kernel client: the kernel future, the global client and its request methods.\nuse super::*;\n\n" + "\n".join([f"//#region {r}" for r in stack] + L[split - 1:])
    # head: drop the header line pair (doc + use super) and keep the rest
    head_body = "\n".join(L[:split - 1]) + "\n" + "\n".join(f"//#endregion {r}" for r in reversed(stack)) + ("\n" if stack else "")
    head_body = head_body.replace("//! 📬️ Kernel replay client: mounted product replay, typed result pages, exchange outcomes, the kernel future and client.", "//! 📬️ Kernel replay vocabulary: mounted product replay admission, recovery and refusal registries, typed operation result pages and the exchange and response slots.", 1)
    # 2. request queue out of the pool
    pool = texts["pool"]
    q = [it for it in items.table(pool) if it["name"] == "KERNEL_REQUEST_QUEUE_CAPACITY"][0]
    q_end = [it for it in items.table(pool) if it["name"].startswith("impl KernelRequestQueue {")][0]
    pool_rest, queue_text, inner = cut(pool, q["start"], q_end["end"])
    print("queue lines", q["start"], q_end["end"], "inner", inner)
    # 3. KernelPoolState out of retained
    ret = texts["retained"]
    kp = [it for it in items.table(ret) if it["name"] == "KernelPoolState"][0]
    rt_rest, kps_text, inner2 = cut(ret, kp["start"], kp["end"])
    print("pool state", kp["start"], kp["end"], "inner", inner2, kp["attrs"])
    # 4. read_native_component out of the parent
    par = texts["parent"]
    rn = [it for it in items.table(par) if it["name"] == "read_native_component"][0]
    par_rest, rnc_text, inner3 = cut(par, rn["start"], rn["end"])
    print("read_native_component", rn["start"], rn["end"])
    out = dict(client_text=client_text, head_body=head_body, pool_rest=pool_rest, queue_text=queue_text, rt_rest=rt_rest, kps_text=kps_text, par_rest=par_rest, rnc_text=rnc_text)
    for k, v in out.items():
        write(RS + "/kernel-" + k + ".rs", v)
    return out


if __name__ == "__main__":
    main("--apply" in sys.argv)
