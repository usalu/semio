#!/usr/bin/env bun
/** 🕰️ G11: is the staged semio-os-mcp fresh against its Cargo dep-info closure? usage: bun g11-mcp-fresh.ts */
import { mcpBinaryFreshness, resolveMcpBinaryPath } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
const binary = resolveMcpBinaryPath("/Users/ueli/Documents/semio");
console.log(binary, JSON.stringify(mcpBinaryFreshness(binary)));
