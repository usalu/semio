import { existsSync, readFileSync, statSync } from "fs";
import { join } from "path";

const repo = "/Users/ueli/Documents/semio";
const paths = {
  flow: "𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow/ lobbies",
};
// use discovered
const candidates = [
  "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow/ lobbies",
];
const files = [
  "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow/