"""🧭️ C12 T6 set — collaborating replicas fold the HUB's order (coordinator P1, 2026-09-29 17:2x; `📓️wp-c12.md` rows 29/30).

Root cause (live probe `c12splice-2`, two humans typing at the same point): each replica kept its own run first and the peer's after
it — the store folds remote operations by their HLC (`ingest_remote`), and the worker's reorder backstop (`remoteFoldedOverLocal`)
keyed on the WORKER's unacknowledged list, which lags the store, so no rebuild fired and the replicas diverged for good.
Now the canonical order is the hub's sequence:
  * a store attached to a transport owner (`Backbones::Port` — browser actor/Shell instance ↔ worker; `Backbones::Channel` —
    native store ↔ sync actor) records the operations it sends as undecided (bounded ledger, `flush_outbound`);
  * `BackboneMessage::Sequenced { envelopes }` (ordinal 5) carries a hub document's remote operations in the hub's order: the
    store places them right before its first undecided operation — splitting a typing run the hub interrupted — and replays the
    undecided ones on top (the rebase); `Mutations` from a peer without a sequencer (a folder/local document) keep the HLC merge;
  * every decided entry is stamped along the sequencer's order (`sequenced_stamp`: its own stamp when later, else the next tick
    after the previous decided entry — derived from the stamp it was SENT with, so every replica holds equal stamps) and the
    undecided entries move past each placed one, because the history fold (`reproject`, every local edit) orders by stamp —
    without it the next local edit re-sorted the rebase back to HLC order (seeded property law, seed 1, 19:5x);
  * `BackboneMessage::Committed { op_ids }` (ordinal 4) decides this replica's own operations where they stand (an ack, a
    lost-ack settle in frame order, a refusal whose rollback follows `Sequenced`);
  * the worker and both sync actors hand every hub frame over run by run (`hubOrderSegmentsV1` / `hub_order_segments`), and
    the worker no longer rebuilds on a reorder. Both flow hub → store only: every guest-egress consumer refuses them.
Guest store + sync twins + plugin binding + host worker/codec/Shell land TOGETHER (a worker sending `sequenced` to an older
guest is refused). Laws: store fixture `🧭️hub-order` (scenarios + seeded property), parity scenarios `hub-order-*` (Rust
actor + TS worker), codec vectors. Idempotent; usage: python3 c12-hub-order-patch.py [--apply]   (default dry run; C12_REPO
overrides the tree)"""
import json
import os
import sys

REPO = os.environ.get("C12_REPO", "/Users/ueli/Documents/semio")
APPLY = "--apply" in sys.argv
STORE_DIR = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/"
STORE = STORE_DIR + "🦀️.rs"
STORE_LAWS = STORE_DIR + "🧪️tests/🔬️unit/🦀️.rs"
SYNC = STORE_DIR + "🔄️sync/🦀️.rs"
WORKER = STORE_DIR + "👷️worker/🟦️.ts"
OS_ROOT = "🧰️framework/🛍️products/💻️os/🟦️.ts"

plan, problems, files = [], [], {}


def read(rel):
    if rel not in files:
        path = os.path.join(REPO, rel)
        files[rel] = open(path, encoding="utf-8").read() if os.path.exists(path) else None
    return files[rel]


def edit(rel, old, new, label):
    text = read(rel)
    if text is None:
        problems.append((label, "missing file"))
    elif (new in text) if old in new else (old not in text):
        plan.append(f"present {label}")
    elif text.count(old) != 1:
        problems.append((label, text.count(old)))
    else:
        files[rel] = text.replace(old, new)
        plan.append(f"edit    {label}")


def create(rel, content, label):
    text = read(rel)
    if text == content:
        plan.append(f"present {label}")
    elif text is not None:
        problems.append((label, "exists with other content"))
    else:
        files[rel] = content
        plan.append(f"create  {label}")


def section(name):
    exec(compile(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections", name), encoding="utf-8").read(), name, "exec"), globals())


for name in sorted(os.listdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), "sections"))):
    if name.endswith(".py"):
        section(name)

for line in plan:
    print(line)
print(len(plan), "operations", len(problems), "problems", problems, "mode=" + ("apply" if APPLY else "dry-run"))
if APPLY and not problems:
    for rel, text in files.items():
        if text is None:
            continue
        path = os.path.join(REPO, rel)
        if not os.path.exists(path) or open(path, encoding="utf-8").read() != text:
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)
    print("written")
