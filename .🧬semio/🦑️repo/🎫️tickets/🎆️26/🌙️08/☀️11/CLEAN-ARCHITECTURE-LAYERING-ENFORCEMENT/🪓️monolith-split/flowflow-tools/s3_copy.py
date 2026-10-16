"""📑 Step 3: the retained-copy file becomes the root of its own crate. Work tree only."""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from ffed import *
C="🧵️retained/📑️copy/🦀️.rs"
t=read(C)
t=replace(t,"//! 🗿️ Retained typed scene copying under an immutable rooted borrow witness.\n","//! 🗿️ Retained typed scene copying under an immutable rooted borrow witness.\n\nuse semio_framework_artifact_flow_flow_document::*;\n")
t=replace(t,"use super::{FlowOwner as Owner, FlowRetirement as Retirement};","use crate::retained::{FlowOwner as Owner, FlowRetirement as Retirement};")
write(C,t)
t=read("🌿️vcs/🦀️.rs")
print(t.count("flow_mutation_retirement"))
print("ok")
