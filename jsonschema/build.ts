#!/usr/bin/env tsx
import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const inputFilePath = join(__dirname, "kit.json");
const outputFilePath = join(__dirname, "kit_unescaped.json");

const jsonContent = readFileSync(inputFilePath, "utf-8");
const unescapedContent = jsonContent.replace(/\\(.)/g, "$1");
writeFileSync(outputFilePath, unescapedContent, "utf-8");

console.log(`✅ Unescaped ${inputFilePath} to ${outputFilePath}`);
