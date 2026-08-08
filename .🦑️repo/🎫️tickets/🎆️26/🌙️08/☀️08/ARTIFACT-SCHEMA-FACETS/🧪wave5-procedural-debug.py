#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
flow_doc = next(root.glob("**/🌊️flow/📄️document/🦀️component.rs"))
text = flow_doc.read_text()
idx = text.find("impl Default for FlowFixture")
print("FILE", flow_doc)
print(text[idx : idx + 1800])

example = next(root.glob("**/🌀️procedural2d/**/🗣️example.dsl.semio"))
print("\nEXAMPLE", example)
print(repr(example.read_text()))

# Print DSL of empty + a rect neuron by reading widget kinds from 3d example style
hex_ex = next(root.glob("**/🗣️hexagonal-mushroom-column.dsl.semio"))
print("\nHEX HEAD")
print("\n".join(hex_ex.read_text().splitlines()[:30]))
