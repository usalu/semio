// #region 🔖Header
// [👤semio📚js💻viteenvd](repo://p/u/semio/b/l/js/f/vite-env.d.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Vite client type declarations for the JavaScript workspace.

// #endregion 🔖Header

// #region 🔖Declarations
// [👤semio📚js💻viteenvd🔖declarations](repo://p/u/semio/b/l/js/f/vite-env.d.ts/s/Declarations)
// Ambient module declarations for non-standard import types.
// Declarations MUST cover all custom asset import suffixes used in the project.

declare module "*.wasm?url" {
  const value: string;
  export default value;
}

declare module "*?raw" {
  const content: string;
  export default content;
}

declare module "*.json?raw" {
  const content: string;
  export default content;
}

interface ImportMetaEnv {
  readonly VITE_APP_TITLE: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
  readonly hot?: {
    readonly data: Record<string, any>;
    accept(callback?: (newModule: unknown) => void): void;
    accept(deps: string[], callback: (modules: unknown[]) => void): void;
    accept(dep: string, callback: (newModule: unknown) => void): void;
    dispose(callback: (data: unknown) => void): void;
    prune(callback: () => void): void;
    decline(): void;
    invalidate(): void;
    on(event: string, callback: (...args: unknown[]) => void): void;
    send(event: string, data?: unknown): void;
  };
  readonly glob: {
    <T = unknown>(pattern: string, options: { eager: true; as?: "raw" | "url"; import?: string; exhaustive?: boolean }): Record<string, T>;
    <T = unknown>(pattern: string, options?: { eager?: false; as?: "raw" | "url"; import?: string; exhaustive?: boolean }): Record<string, () => Promise<T>>;
  };
}
