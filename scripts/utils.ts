import { ChildProcess, execSync, spawn } from "child_process";
import { readdirSync, readFileSync, renameSync, statSync, unlinkSync, writeFileSync } from "fs";
import { join } from "path";
import sharp from "sharp";

//#region Image Processing
export async function resizeImage(
  sourcePath: string,
  targetPathBase: string,
  targetResolutions: number[]
): Promise<void> {
  for (const resolution of targetResolutions) {
    const targetPath = `${targetPathBase}_${resolution}x${resolution}.png`;
    await sharp(sourcePath)
      .resize(resolution, resolution)
      .png()
      .toFile(targetPath);
  }
}
//#endregion

//#region File Operations
export function renameFilesByPattern(pattern: RegExp, replacement: string, rootDir: string = "."): void {
  function walk(dir: string): void {
    const files = readdirSync(dir);
    for (const file of files) {
      const filePath = join(dir, file);
      const stat = statSync(filePath);
      if (stat.isDirectory()) {
        walk(filePath);
      } else {
        const newPath = filePath.replace(pattern, replacement);
        if (newPath !== filePath) {
          renameSync(filePath, newPath);
        }
      }
    }
  }
  walk(rootDir);
}

export function deleteFilesByPattern(pattern: string, rootDir: string = "."): void {
  function walk(dir: string): void {
    const files = readdirSync(dir);
    for (const file of files) {
      const filePath = join(dir, file);
      const stat = statSync(filePath);
      if (stat.isDirectory()) {
        walk(filePath);
      } else if (file.match(new RegExp(pattern))) {
        unlinkSync(filePath);
      }
    }
  }
  walk(rootDir);
}
//#endregion

//#region Process Management
export function stopProcessOnPort(port: number): void {
  try {
    const output = execSync(`netstat -ano`, { encoding: "utf-8" });
    const lines = output.split("\n");
    for (const line of lines) {
      if (line.includes(`:${port}`) && line.includes("LISTENING")) {
        const parts = line.trim().split(/\s+/);
        const pid = parts[parts.length - 1];
        if (pid && !isNaN(Number(pid))) {
          execSync(`taskkill /F /PID ${pid}`, { stdio: "ignore" });
          break;
        }
      }
    }
  } catch {
    // Process might not be running
  }
}

export function runProcess(
  command: string,
  args: string[],
  options?: { cwd?: string; onExit?: () => void }
): ChildProcess {
  const proc = spawn(command, args, {
    cwd: options?.cwd,
    stdio: "inherit",
    shell: true,
  });

  if (options?.onExit) {
    proc.on("exit", options.onExit);
    process.on("SIGINT", () => {
      proc.kill();
      options.onExit?.();
    });
  }

  return proc;
}
//#endregion

//#region JSON Utilities
export function unescapeJson(inputPath: string, outputPath: string): void {
  const content = readFileSync(inputPath, "utf-8");
  const unescaped = content.replace(/\\(.)/g, "$1");
  writeFileSync(outputPath, unescaped, "utf-8");
}
//#endregion
