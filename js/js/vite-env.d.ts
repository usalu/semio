/// <reference types="vite/client" />
/// <reference types="vitest/globals" />

declare module "*.wasm?url" {
  const value: string;
  export default value;
}
