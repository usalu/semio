# Release build fvdpKd2

## Summary

| Metric | Value |
|---|---|
| Wall | 529.9 s |
| Compiled units | 144 |
| Failed units | 0 |
| Unit CPU | 1.07 CPU-h |
| Mean CPU (whole build) | 95.1 % |
| Peak rustc resident (sum) | 14.54 GiB |
| Frozen samples | 0 |
| Peak swap used | 4.39 GiB |

## Phases

| Phase | From s | To s | Mean CPU |
|---|---|---|---|
| head | 0.0 | 4.6 | 66.7 % |
| compile | 4.6 | 520.9 | 95.5 % |
| tail | 520.9 | 529.9 | 84.2 % |

## Passes

| Pass | Profile | Target | Start s | Wall s | Exit | Failed packages |
|---|---|---|---|---|---|---|
| wasm-release-subset | wasm-release | wasm32-wasip2 | 3.8 | 517.3 | 0 | - |

## Critical path

| Unit | Unblocked by | Ready s | Start s | Wait s | Duration s |
|---|---|---|---|---|---|
| proc-macro2@build_script_build (bin, host, wasm-release) | root | 4.6 | 4.6 | 0.0 | 0.4 |
| proc-macro2 build script (run) | full | 5.0 | 5.0 | 0.0 | 0.3 |
| proc-macro2@proc_macro2 (lib, host, wasm-release) | run | 5.4 | 5.4 | 0.0 | 0.4 |
| quote@quote (lib, host, wasm-release) | rmeta | 5.6 | 5.7 | 0.0 | 0.2 |
| syn@syn (lib, host, wasm-release) | rmeta | 5.8 | 6.4 | 0.6 | 3.3 |
| semio-framework-value-derive@semio_framework_value_derive (proc-macro, host, wasm-release) | full | 9.7 | 9.7 | 0.0 | 2.1 |
| semio-framework-value@semio_framework_value (lib, wasm32-wasip2, wasm-release) | full | 11.8 | 11.8 | 0.0 | 6.3 |
| semio-framework-diagnostic@semio_framework_diagnostic (lib, wasm32-wasip2, wasm-release) | rmeta | 13.6 | 13.7 | 0.0 | 2.8 |
| semio-framework-pack-error@semio_framework_pack_error (lib, wasm32-wasip2, wasm-release) | rmeta | 14.1 | 14.6 | 0.5 | 0.4 |
| semio-framework-replication@protocol (lib, wasm32-wasip2, wasm-release) | rmeta | 14.8 | 15.0 | 0.3 | 12.1 |
| semio-framework-pack@pack (lib, wasm32-wasip2, wasm-release) | rmeta | 20.1 | 20.7 | 0.5 | 6.5 |
| semio-framework-os-kernel@semio_framework_os_kernel (rlib,cdylib, wasm32-wasip2, wasm-release) | full | 27.2 | 27.3 | 0.1 | 28.5 |
| semio-framework-plugin@semio_framework_plugin (lib, wasm32-wasip2, wasm-release) | full | 55.8 | 55.8 | 0.0 | 31.2 |
| semio-s-artifact-stdio-contract@semio_s_artifact_stdio_contract (lib, wasm32-wasip2, wasm-release) | rmeta | 65.5 | 65.6 | 0.1 | 18.0 |
| semio-s-artifact-stdio-binary@semio_s_artifact_stdio_binary (lib, wasm32-wasip2, wasm-release) | rmeta | 69.2 | 69.3 | 0.1 | 62.9 |
| semio-s-artifact-stdio-gif@semio_s_artifact_stdio_gif (lib, wasm32-wasip2, wasm-release) | rmeta | 75.1 | 153.6 | 78.4 | 39.6 |
| semio-hub-stdio-image@semio_hub_stdio_image (cdylib,rlib, wasm32-wasip2, wasm-release) | full | 193.2 | 275.2 | 82.0 | 245.6 |

Critical path length: 516.3 s.

## Top 20 units by CPU

| Unit | CPU s | Wall s | Peak GiB | Admission wait s |
|---|---|---|---|---|
| semio-hub-stdio-image@semio_hub_stdio_image (cdylib,rlib, wasm32-wasip2, wasm-release) | 749.2 | 245.6 | 9.28 | 45.9 |
| semio-hub-stdio-office@semio_hub_stdio_office (cdylib,rlib, wasm32-wasip2, wasm-release) | 521.9 | 183.1 | 8.26 | 0.0 |
| semio-hub-stdio-mesh@semio_hub_stdio_mesh (cdylib,rlib, wasm32-wasip2, wasm-release) | 355.9 | 136.6 | 4.95 | 70.5 |
| semio-hub-stdio-binary@semio_hub_stdio_binary (cdylib,rlib, wasm32-wasip2, wasm-release) | 322.4 | 141.6 | 3.85 | 0.0 |
| semio-hub-stdio-media@semio_hub_stdio_media (cdylib,rlib, wasm32-wasip2, wasm-release) | 306.3 | 126.1 | 3.73 | 17.2 |
| semio-s-artifact-stdio-gltf@semio_s_artifact_stdio_gltf (lib, wasm32-wasip2, wasm-release) | 169.5 | 131.8 | 2.76 | 0.0 |
| semio-framework-os-kernel@semio_framework_os_kernel (rlib,cdylib, wasm32-wasip2, wasm-release) | 100.0 | 28.5 | 2.99 | 0.0 |
| semio-framework-plugin@semio_framework_plugin (lib, wasm32-wasip2, wasm-release) | 78.6 | 31.2 | 2.50 | 0.0 |
| semio-framework@semio_framework (lib, wasm32-wasip2, wasm-release) | 67.0 | 30.5 | 2.13 | 0.0 |
| semio-s-artifact-stdio-gif@semio_s_artifact_stdio_gif (lib, wasm32-wasip2, wasm-release) | 62.5 | 39.6 | 1.32 | 0.0 |
| semio-s-artifact-stdio-binary@semio_s_artifact_stdio_binary (lib, wasm32-wasip2, wasm-release) | 59.9 | 62.9 | 1.01 | 0.0 |
| semio-s-artifact-stdio-svg@semio_s_artifact_stdio_svg (lib, wasm32-wasip2, wasm-release) | 52.3 | 54.3 | 1.09 | 0.0 |
| semio-s-artifact-stdio-txt@semio_s_artifact_stdio_txt (lib, wasm32-wasip2, wasm-release) | 49.8 | 57.8 | 0.94 | 0.0 |
| semio-s-artifact-stdio-mp4@semio_s_artifact_stdio_mp4 (lib, wasm32-wasip2, wasm-release) | 48.6 | 55.2 | 1.08 | 0.0 |
| semio-s-artifact-stdio-zip@semio_s_artifact_stdio_zip (lib, wasm32-wasip2, wasm-release) | 46.6 | 52.6 | 0.96 | 0.0 |
| semio-s-artifact-stdio-obj@semio_s_artifact_stdio_obj (lib, wasm32-wasip2, wasm-release) | 45.6 | 48.1 | 0.96 | 0.0 |
| semio-s-artifact-stdio-xml@semio_s_artifact_stdio_xml (lib, wasm32-wasip2, wasm-release) | 42.8 | 48.2 | 0.90 | 0.0 |
| semio-s-artifact-stdio-tiff@semio_s_artifact_stdio_tiff (lib, wasm32-wasip2, wasm-release) | 42.5 | 46.9 | 0.91 | 0.0 |
| semio-s-artifact-stdio-docx@semio_s_artifact_stdio_docx (lib, wasm32-wasip2, wasm-release) | 40.1 | 43.3 | 0.93 | 0.0 |
| semio-s-artifact-stdio-png@semio_s_artifact_stdio_png (lib, wasm32-wasip2, wasm-release) | 39.3 | 43.7 | 0.90 | 0.0 |

## Top 20 units by peak

| Unit | Peak GiB | Predicted GiB | Wall s |
|---|---|---|---|
| semio-hub-stdio-image@semio_hub_stdio_image (cdylib,rlib, wasm32-wasip2, wasm-release) | 9.28 | 6.00 | 245.6 |
| semio-hub-stdio-office@semio_hub_stdio_office (cdylib,rlib, wasm32-wasip2, wasm-release) | 8.26 | 6.00 | 183.1 |
| semio-hub-stdio-mesh@semio_hub_stdio_mesh (cdylib,rlib, wasm32-wasip2, wasm-release) | 4.95 | 6.00 | 136.6 |
| semio-hub-stdio-binary@semio_hub_stdio_binary (cdylib,rlib, wasm32-wasip2, wasm-release) | 3.85 | 6.00 | 141.6 |
| semio-hub-stdio-media@semio_hub_stdio_media (cdylib,rlib, wasm32-wasip2, wasm-release) | 3.73 | 6.00 | 126.1 |
| semio-framework-os-kernel@semio_framework_os_kernel (rlib,cdylib, wasm32-wasip2, wasm-release) | 2.99 | 6.00 | 28.5 |
| semio-s-artifact-stdio-gltf@semio_s_artifact_stdio_gltf (lib, wasm32-wasip2, wasm-release) | 2.76 | 2.23 | 131.8 |
| semio-framework-plugin@semio_framework_plugin (lib, wasm32-wasip2, wasm-release) | 2.50 | 0.60 | 31.2 |
| semio-framework@semio_framework (lib, wasm32-wasip2, wasm-release) | 2.13 | 0.61 | 30.5 |
| semio-s-artifact-stdio-gif@semio_s_artifact_stdio_gif (lib, wasm32-wasip2, wasm-release) | 1.32 | 1.18 | 39.6 |
| semio-framework-ui@semio_framework_ui (lib, wasm32-wasip2, wasm-release) | 1.18 | 0.60 | 13.4 |
| semio-s-artifact-stdio-svg@semio_s_artifact_stdio_svg (lib, wasm32-wasip2, wasm-release) | 1.09 | 1.16 | 54.3 |
| semio-s-artifact-stdio-mp4@semio_s_artifact_stdio_mp4 (lib, wasm32-wasip2, wasm-release) | 1.08 | 0.88 | 55.2 |
| semio-s-artifact-stdio-binary@semio_s_artifact_stdio_binary (lib, wasm32-wasip2, wasm-release) | 1.01 | 0.77 | 62.9 |
| semio-s-artifact-stdio-zip@semio_s_artifact_stdio_zip (lib, wasm32-wasip2, wasm-release) | 0.96 | 1.05 | 52.6 |
| semio-s-artifact-stdio-obj@semio_s_artifact_stdio_obj (lib, wasm32-wasip2, wasm-release) | 0.96 | 0.97 | 48.1 |
| semio-s-artifact-stdio-txt@semio_s_artifact_stdio_txt (lib, wasm32-wasip2, wasm-release) | 0.94 | 0.85 | 57.8 |
| semio-s-artifact-stdio-docx@semio_s_artifact_stdio_docx (lib, wasm32-wasip2, wasm-release) | 0.93 | 1.29 | 43.3 |
| semio-s-artifact-stdio-tiff@semio_s_artifact_stdio_tiff (lib, wasm32-wasip2, wasm-release) | 0.91 | 0.96 | 46.9 |
| semio-s-artifact-stdio-png@semio_s_artifact_stdio_png (lib, wasm32-wasip2, wasm-release) | 0.90 | 0.90 | 43.7 |

## Idle windows

| From s | To s | Mean CPU | Cause |
|---|---|---|---|

## Relink fan-out (L9)

| Library | Components | Mean component unit s | Fan-out cost s |
|---|---|---|---|
| semio-framework-hash@semio_framework_hash (wasm-release) | 14 | 124.3 | 1740.6 |
| itoa@itoa (wasm-release) | 13 | 133.2 | 1731.8 |
| memchr@memchr (wasm-release) | 13 | 133.2 | 1731.8 |
| semio-framework-io-base64@semio_framework_io_base64 (wasm-release) | 13 | 133.2 | 1731.8 |
| semio-framework-schema-state@semio_framework_schema_state (wasm-release) | 13 | 133.2 | 1731.8 |
| semio-framework-value@semio_framework_value (wasm-release) | 13 | 133.2 | 1731.8 |
| serde_core@serde_core (wasm-release) | 13 | 133.2 | 1731.8 |
| serde_json@serde_json (wasm-release) | 13 | 133.2 | 1731.8 |
| serde@serde (wasm-release) | 13 | 133.2 | 1731.8 |
| zmij@zmij (wasm-release) | 13 | 133.2 | 1731.8 |
| semio-framework-ui-locale@semio_framework_ui_locale (wasm-release) | 12 | 143.6 | 1723.0 |
| semio-framework-schema-composition@semio_framework_schema_composition (wasm-release) | 11 | 154.0 | 1694.5 |
| proc-macro2@proc_macro2 (wasm-release) | 7 | 124.3 | 870.3 |
| quote@quote (wasm-release) | 7 | 124.3 | 870.3 |
| semio-framework-async@semio_framework_async (wasm-release) | 7 | 124.3 | 870.3 |

## Stage overlap

6 stage(s), 8.8 s of stage time, 0 % overlapped the compile phase.
