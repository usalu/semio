#!/usr/bin/env bun
/** 🏗️ Assembles the six Go MCP source files into one godfile with the seven ticket-mandated regions. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const SOURCE = process.argv[2];
const TARGET = process.argv[3];

/** ✂️ Strips the header region, the package clause, the import block and every inner region marker. */
function body(file: string, keep?: string[]): string {
  const text = readFileSync(join(SOURCE, file), "utf8");
  const lines = text.split("\n");
  const out: string[] = [];
  let inHeader = false;
  let inImports = false;
  let current = "";
  for (const line of lines) {
    if (line.startsWith("// #region ")) current = line.slice("// #region ".length).trim();
    if (keep && !line.startsWith("// #region ") && !keep.includes(current)) {
      if (line.startsWith("// #endregion ")) current = "";
      continue;
    }
    if (keep && line.startsWith("// #region ") && !keep.includes(current)) continue;
    if (line.startsWith("// #region 🧲️Header")) {
      inHeader = true;
      continue;
    }
    if (inHeader) {
      if (line.startsWith("// #endregion 🧲️Header")) inHeader = false;
      continue;
    }
    if (line.startsWith("package main")) continue;
    if (line.startsWith("import (")) {
      inImports = true;
      continue;
    }
    if (inImports) {
      if (line.startsWith(")")) inImports = false;
      continue;
    }
    if (line.startsWith("// #region ") || line.startsWith("// #endregion ")) continue;
    out.push(line);
  }
  return out.join("\n").replace(/\n{3,}/g, "\n\n").trim();
}

const IMPORTS = `import (
	"bufio"
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strconv"
	"strings"
	"sync"
	"sync/atomic"

	"github.com/usalu/semio/repo/client"
)`;

type Source = [file: string, keep?: string[]];

const REGIONS: Array<[string, Source[]]> = [
  ["📜️Protocol", [["📜️protocol.go"]]],
  ["📡️Event", [["📡️event.go"]]],
  ["🚚️Transport", [["🚚️transport.go"]]],
  ["🔐️Session", [["🖥️server.go", ["⚙️Configuration", "🖥️Server", "🔐Session"]]]],
  ["🚦️Routing", [["🖥️server.go", ["🚦Routing", "📦️Encoding"]]]],
  ["🗄️Repository", [["🗄️repository.go"]]],
  ["🦀️Entrypoint", [["🧩️component.go"]]],
];

const parts: string[] = [
  "// #region 🧲️Header",
  "",
  "// 2025-2026 Ueli Saluz <ueli@semio-tech.com>",
  "",
  "// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.",
  "",
  "// Package main owns the repo Model Context Protocol server: the bounded JSON-RPC contract, the hash-chained event log, the line-delimited stdio transport, the session state machine, request routing and the repository tool/resource/prompt surface.",
  "",
  "// #endregion 🧲️Header",
  "",
  "package main",
  "",
  IMPORTS,
  "",
];

for (const [name, files] of REGIONS) {
  parts.push(`// #region ${name}`, "");
  for (const [file, keep] of files) parts.push(body(file, keep), "");
  parts.push(`// #endregion ${name}`, "");
}

writeFileSync(TARGET, parts.join("\n"));
console.log(`[assemble] wrote ${TARGET}`);
