"""statutes: lift the catalog, ignore directives, breach, specification text, sources, analyze, autofix, digest, deflate and breach-cache regions of 📜️statutes into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from plan_common import *

SPEC = dict(
    target="statutes",
    gate_members=True,
    imports=(13, 16),
    root_drop=[],
    skip_import=[],
    root_cfg={},
    children=[
        C(kind="cluster", mod="catalog", folder="📚️catalog", ranges=[(22, 93)],
          doc=["//! 📚️ The closed statute catalog: the embedded document, the lazy statute and policy indexes and the lookup functions."]),
        C(kind="cluster", mod="ignore_directives", folder="🙈️ignore-directives", ranges=[(94, 152)],
          doc=["//! 🙈️ Ignore directives: parsing `// compose-ignore-` lines and filtering the breaches they suppress."]),
        C(kind="cluster", mod="breach", folder="🔶️breach", ranges=[(153, 184)],
          doc=["//! 🔶️ Breach construction: the canonical breach identifier and the breach record."]),
        C(kind="cluster", mod="specification_text", folder="📐️specification-text", ranges=[(185, 319)],
          doc=["//! 📐️ Specification text: spec keyword detection and the implementation-syntax leak scan."]),
        C(kind="cluster", mod="sources", folder="🗂️sources", ranges=[(320, 436)],
          doc=["//! 🗂️ Sources: the analysed files, their parsed sections and the definition-head helpers."]),
        C(kind="cluster", mod="analyze", folder="🔍️analyze", ranges=[(437, 1247)],
          doc=["//! 🔍️ Analysis: header, section, orphan, definition and requirement policies over a source set."]),
        C(kind="cluster", mod="autofix", folder="🩹️autofix", ranges=[(1248, 1294)],
          doc=["//! 🩹️ Autofix: the repairs applied to one source and the statutes they fixed."]),
        C(kind="cluster", mod="digest", folder="🔏️digest", ranges=[(1295, 1359)],
          doc=["//! 🔏️ Digest: the hand-rolled SHA-256 and its hexadecimal form."]),
        C(kind="cluster", mod="deflate", folder="🗜️deflate", ranges=[(1360, 1650)],
          doc=["//! 🗜️ Deflate: the stored-block gzip encoder and the inflating gzip decoder with CRC-32 verification."]),
        C(kind="cluster", mod="breach_cache", folder="🧷️breach-cache", ranges=[(1651, 1700)],
          doc=["//! 🧷️ Breach cache: the on-disk envelope, its digest and its plain and compressed encodings."]),
    ],
)

if __name__ == "__main__":
    run(SPEC)
