---
slug: STATE-MACHINE-REFACTOR
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: >-
  Refactor to single global sketchpad machine with open/closed plugin
  architecture
model: claude-opus-4.5
input:
  - prompt: >-
      Refactor the state machines: Currently there are two machines being used
      (createMachine). There should be only one global sketchpad machine. All
      app specific logic should be part of the APP.tsx files. There should be no
      design, type, etc logic part of Sketchpad.tsx file. All should follow
      open/closed principle.
    date: '2025-12-15T12:45:50.535Z'
commit: 76900221ecf5cfb30a37d69fbb66abb3e0a0e45a
files: {}
lines:
  added: 0
  removed: 0
---
# Previously

# Plan

# Changes
