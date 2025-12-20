---
slug: DECOUPLE-STORES-FROM-YJS
prompt: >-
  All app stores are still entangled with yjs. Only the kit store should use
  yjs. All other stores should use the state machine for state management. E.g.
  the AppStore should not use yMap such as in the constructor. Make sure yjs
  (yMap, yArray) dont appear anywhere outside of kit store.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-20T11:39:09.443Z'
iterations: []
---
# Previously

# Plan

# Changes
