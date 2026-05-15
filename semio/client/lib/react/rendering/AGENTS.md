---
technology: semio
bundle:
 name: ui
 emoji: 🖼️
 description: A VSCode extension for interacting with semio.
 kind: ui
---

# 🧾 Specification

## Strict layering (semio UI)

- Kit-bearing UI MUST depend on **`@semio/react`** hooks and props, not on **`@semio/js`** directly.
- Storybook / Vite may resolve `@semio/js` only for transitive tooling; application code SHOULD stay on the react layer.

## 🕸️ Systems

## 🧮 Algorithms

## 🛠️ Mechanisms

## 📛 Entities

### Components

A `component` MUST work with full/partial controlled/uncontrolled state managment.
