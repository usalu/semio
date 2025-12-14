/// <reference types="@electron-forge/plugin-vite/forge-vite-env" />
declare module "@electron/fuses" {
  export enum FuseVersion {
    V1 = "V1",
  }
  export enum FuseV1Options {
    RunAsNode = "RunAsNode",
    EnableCookieEncryption = "EnableCookieEncryption",
    EnableNodeOptionsEnvironmentVariable = "EnableNodeOptionsEnvironmentVariable",
    EnableNodeCliInspectArguments = "EnableNodeCliInspectArguments",
    EnableEmbeddedAsarIntegrityValidation = "EnableEmbeddedAsarIntegrityValidation",
    OnlyLoadAppFromAsar = "OnlyLoadAppFromAsar",
  }
}
