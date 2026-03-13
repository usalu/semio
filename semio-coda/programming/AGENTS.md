# 🧾 Specification

## 🕸️ Systems

### Project, Program, Space, Adjacency

A **space program** defines area constraints (min/max) per program kind. An **adjacency matrix** defines required or desired adjacencies between program kinds (mandatory, desirable, neutral, negative).

## 🛠️ Mechanisms

- Go validator (`programming/go`) validates programming translations against `.progam/config.json` or `.coda/programming-requirements.json`.
- Space program: `byKind[].constraints.min` / `max` vs `totals[kind]`.
- Adjacency matrix: `adjacency[].type=mandatory` requires each room of `from` kind to be adjacent to at least one room of `to` kind.

## 📛 Concepts

### Project

### Program

### Space
