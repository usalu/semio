"""move: lift the workspace, plan, executor, casing, language helpers, sealed, rename, folders, files, agents docs, sections, integrate and extract regions of 🚚️move into children."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from plan_common import *

SPEC = dict(
    target="move",
    gate_members=True,
    imports=(15, 21),
    root_drop=[],
    skip_import=[],
    root_cfg={},
    children=[
        C(kind="cluster", mod="workspace", folder="🗄️workspace", ranges=[(27, 163)],
          doc=["//! 🗄️ Workspace: what a workspace path holds and the in-memory workspace every operation reads."]),
        C(kind="cluster", mod="plan", folder="🧾️plan", ranges=[(164, 253)],
          doc=["//! 🧾️ Plan: the change set an operation returns."]),
        C(kind="cluster", mod="executor", folder="🎬️executor", ranges=[(254, 366)],
          doc=["//! 🎬️ Executor: applying a plan to a workspace or a disk."]),
        C(kind="cluster", mod="casing", folder="🔤️casing", ranges=[(367, 437)],
          doc=["//! 🔤️ Casing: the casing variants of a renamed token."]),
        C(kind="cluster", mod="language_helpers", folder="🗣️language-helpers", ranges=[(438, 685)],
          doc=["//! 🗣️ Language helpers: section markers and patterns per source language."]),
        C(kind="cluster", mod="sealed", folder="🔏️sealed", ranges=[(686, 729)],
          doc=["//! 🔏️ Sealed: operations that refuse to touch sealed content."]),
        C(kind="cluster", mod="rename", folder="✏️rename", ranges=[(730, 850)],
          doc=["//! ✏️ Rename: renaming a token in all of its casing variants."]),
        C(kind="cluster", mod="folders", folder="📁️folders", ranges=[(851, 903)],
          doc=["//! 📁️ Folders: creating, moving and deleting folders."]),
        C(kind="cluster", mod="files", folder="📄️files", ranges=[(904, 956)],
          doc=["//! 📄️ Files: creating, moving and deleting files."]),
        C(kind="cluster", mod="agents_docs", folder="📚️agents-docs", ranges=[(957, 1024)],
          doc=["//! 📚️ Agents docs: keeping the agents documents of moved folders consistent."]),
        C(kind="cluster", mod="sections", folder="📑️sections", ranges=[(1025, 1128)],
          doc=["//! 📑️ Sections: creating, renaming and deleting sections."]),
        C(kind="cluster", mod="integrate", folder="📥️integrate", ranges=[(1129, 1230)],
          doc=["//! 📥️ Integrate: moving a file into a section."]),
        C(kind="cluster", mod="extract", folder="🧲️extract", ranges=[(1231, 1313)],
          doc=["//! 🧲️ Extract: moving a section into its own file."]),
    ],
)

if __name__ == "__main__":
    run(SPEC)
