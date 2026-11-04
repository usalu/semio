import fs from "node:fs";
import { dirname, resolve } from "node:path";

function compatibleLibraries(directory, version, checked) {
  if (!version) return true;
  const [major, minor] = version.split(".").map(Number);
  return fs.readdirSync(directory).filter(name => /\.so(?:\.|$)/.test(name)).every(name => {
    const path = fs.realpathSync(resolve(directory, name));
    if (!fs.statSync(path).isFile()) return true;
    if (!checked.has(path)) {
      const versions = fs.readFileSync(path).toString("latin1").matchAll(/GLIBC_(\d+)\.(\d+)/g);
      checked.set(path, ![...versions].some(([, a, b]) => Number(a) > major || Number(a) === major && Number(b) > minor));
    }
    return checked.get(path);
  });
}

export function browserOptions() {
  const cache = resolve(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright");
  const shell = fs.existsSync(cache) && fs.readdirSync(cache).find((name) => name.startsWith("chromium_headless_shell-"));
  const options = { headless: true };
  if (shell) options.executablePath = resolve(cache, shell, "chrome-headless-shell-linux64/chrome-headless-shell");
  if (process.platform === "linux" && fs.existsSync("/nix/store")) {
    const checked = new Map();
    const report = process.report.getReport();
    const version = report.header.glibcVersionRuntime;
    const libc = report.sharedObjects.find(path => path.endsWith("/libc.so.6"));
    const libraries = fs.readdirSync("/nix/store")
      .filter((name) => /(glib-|nss-|nspr-|dbus-|cups-|libdrm-|libxkbcommon-|at-spi|atk-|libx11-|libxcb-|libxcomposite-|libxdamage-|libxext-|libxfixes-|libxrandr-|mesa-|libgbm-|expat-|alsa-lib-|pango-|cairo-)/i.test(name))
      .map((name) => `/nix/store/${name}/lib`).filter((path) => fs.existsSync(path) && compatibleLibraries(path, version, checked));
    options.env = { ...process.env, LD_LIBRARY_PATH: [...(libc ? [dirname(libc)] : []), ...libraries].join(":") };
  }
  return options;
}
