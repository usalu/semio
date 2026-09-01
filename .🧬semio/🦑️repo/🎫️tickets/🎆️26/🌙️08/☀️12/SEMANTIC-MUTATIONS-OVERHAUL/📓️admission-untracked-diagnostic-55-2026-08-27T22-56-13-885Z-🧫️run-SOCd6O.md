# 🧾️ Exact Untracked Git Diagnostic 55

New diagnostic capture. Helper outcome: threw. Complete raw bytes, exact arguments, source fingerprints, and predicate reasons follow. No candidate path was used for a filesystem probe.

JSON SHA256: 70dd2fe98baec0b4412fb00f168d28ac31a98f7a4d831401a4fbe54ff9983c9c

```json
{
  "schemaVersion": 1,
  "claim": "Instrumented original-loader diagnostic, not an unmodified public-wrapper replay or production release. One requested untracked Git call; returned candidate paths are inspected only as in-memory text/bytes. No trimming/filtering or candidate filesystem probes.",
  "startedAt": "2026-08-27T22:56:13.885Z",
  "completedAt": "2026-08-27T22:56:18.825Z",
  "workspace": "/Users/ueli/Documents/semio",
  "run": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O",
  "ticketReceipt": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55-2026-08-27T22-56-13-885Z-🧫️run-SOCd6O.md",
  "runtime": {
    "bunVersion": "1.3.14",
    "executable": "/Users/ueli/.bun/bin/bun",
    "platform": "darwin",
    "nofollowFlag": 256
  },
  "sources": {
    "controller": {
      "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/📜️script.ts",
      "first": {
        "device": "16777230",
        "inode": "134807844",
        "mode": "33188",
        "size": "19151",
        "modifiedNs": "1787871346723574547",
        "changedNs": "1787871346723574547",
        "sha256": "6abe63e12f47256cdad74d819de94f00aafe948218a6e72be99b4364dcbf82bf",
        "bytes": 19151
      },
      "snapshot": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O/🧬️controller.source.txt",
      "endpoints": [
        {
          "phase": "after-instrumented-import",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "134807844",
            "mode": "33188",
            "size": "19151",
            "modifiedNs": "1787871346723574547",
            "changedNs": "1787871346723574547",
            "sha256": "6abe63e12f47256cdad74d819de94f00aafe948218a6e72be99b4364dcbf82bf",
            "bytes": 19151
          }
        },
        {
          "phase": "before-one-git-call",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "134807844",
            "mode": "33188",
            "size": "19151",
            "modifiedNs": "1787871346723574547",
            "changedNs": "1787871346723574547",
            "sha256": "6abe63e12f47256cdad74d819de94f00aafe948218a6e72be99b4364dcbf82bf",
            "bytes": 19151
          }
        },
        {
          "phase": "final",
          "stable": false,
          "skipped": "Fixed source is also a returned candidate; no candidate-path recapture permitted."
        }
      ]
    },
    "N": {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts",
      "first": {
        "device": "16777230",
        "inode": "129693956",
        "mode": "33188",
        "size": "896572",
        "modifiedNs": "1787869282950467799",
        "changedNs": "1787869282950467799",
        "sha256": "0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce",
        "bytes": 896572
      },
      "snapshot": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O/🧬️N.source.txt",
      "endpoints": [
        {
          "phase": "after-instrumented-import",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "129693956",
            "mode": "33188",
            "size": "896572",
            "modifiedNs": "1787869282950467799",
            "changedNs": "1787869282950467799",
            "sha256": "0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce",
            "bytes": 896572
          }
        },
        {
          "phase": "before-one-git-call",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "129693956",
            "mode": "33188",
            "size": "896572",
            "modifiedNs": "1787869282950467799",
            "changedNs": "1787869282950467799",
            "sha256": "0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce",
            "bytes": 896572
          }
        },
        {
          "phase": "final",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "129693956",
            "mode": "33188",
            "size": "896572",
            "modifiedNs": "1787869282950467799",
            "changedNs": "1787869282950467799",
            "sha256": "0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce",
            "bytes": 896572
          }
        }
      ]
    },
    "D": {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts",
      "first": {
        "device": "16777230",
        "inode": "109708023",
        "mode": "33188",
        "size": "655775",
        "modifiedNs": "1787858918843947200",
        "changedNs": "1787858918843947200",
        "sha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423",
        "bytes": 655775
      },
      "snapshot": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O/🧬️D.source.txt",
      "endpoints": [
        {
          "phase": "after-instrumented-import",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "109708023",
            "mode": "33188",
            "size": "655775",
            "modifiedNs": "1787858918843947200",
            "changedNs": "1787858918843947200",
            "sha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423",
            "bytes": 655775
          }
        },
        {
          "phase": "before-one-git-call",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "109708023",
            "mode": "33188",
            "size": "655775",
            "modifiedNs": "1787858918843947200",
            "changedNs": "1787858918843947200",
            "sha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423",
            "bytes": 655775
          }
        },
        {
          "phase": "final",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "109708023",
            "mode": "33188",
            "size": "655775",
            "modifiedNs": "1787858918843947200",
            "changedNs": "1787858918843947200",
            "sha256": "807e744e080d7d4fcefe61da035870a9e04fe7e8189631d9c0056290c94f0423",
            "bytes": 655775
          }
        }
      ]
    },
    "taxonomy": {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
      "first": {
        "device": "16777230",
        "inode": "129218887",
        "mode": "33188",
        "size": "386042",
        "modifiedNs": "1787871076158192537",
        "changedNs": "1787871076158192537",
        "sha256": "84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce",
        "bytes": 386042
      },
      "snapshot": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O/🧬️taxonomy.source.txt",
      "endpoints": [
        {
          "phase": "after-instrumented-import",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "129218887",
            "mode": "33188",
            "size": "386042",
            "modifiedNs": "1787871076158192537",
            "changedNs": "1787871076158192537",
            "sha256": "84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce",
            "bytes": 386042
          }
        },
        {
          "phase": "before-one-git-call",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "129218887",
            "mode": "33188",
            "size": "386042",
            "modifiedNs": "1787871076158192537",
            "changedNs": "1787871076158192537",
            "sha256": "84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce",
            "bytes": 386042
          }
        },
        {
          "phase": "final",
          "stable": true,
          "fingerprint": {
            "device": "16777230",
            "inode": "129218887",
            "mode": "33188",
            "size": "386042",
            "modifiedNs": "1787871076158192537",
            "changedNs": "1787871076158192537",
            "sha256": "84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce",
            "bytes": 386042
          }
        }
      ]
    }
  },
  "moduleEvidence": {
    "originalModuleUrl": "file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%A6%91%EF%B8%8Frepo/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%9A%EF%B8%8Flibrary/%F0%9F%A7%B9%EF%B8%8Fnormalization/%F0%9F%9F%A6%EF%B8%8F.ts",
    "originalResolveDir": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization",
    "originalSha256": "0612b679b15d2d1b723ab81764c1ee654711ad6ea04e2d4168645692342dcdce",
    "appendedExport": "\nexport { loadTaxonomy };\nexport const __admission55ModuleIdentity = { url: import.meta.url, path: import.meta.path, dir: import.meta.dir };\n",
    "appendedExportSha256": "df9881c60fc730507875d70ecff032f1347dced76c04a4b348f1ff8812ae51e2",
    "instrumentedSha256": "f9c6fecb35b395e84e2c46a3878fb87c752d2dd34e520953150fc7b4310944cc",
    "loadCount": 1,
    "identity": {
      "url": "file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%A6%91%EF%B8%8Frepo/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%9A%EF%B8%8Flibrary/%F0%9F%A7%B9%EF%B8%8Fnormalization/%F0%9F%9F%A6%EF%B8%8F.ts",
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts",
      "dir": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization"
    },
    "identityPreserved": true
  },
  "astEvidence": {
    "slices": [
      {
        "name": "sourceAdmissionSafePath",
        "text": "function sourceAdmissionSafePath(path: string): boolean {\n  return path.length > 0 && !path.startsWith(\"/\") && !/^[A-Za-z]:/u.test(path) && !path.includes(\"\\\\\") && !/[\\u0000-\\u001f\\u007f]/u.test(path) && Buffer.from(path).toString(\"utf8\") === path && path.split(\"/\").every((part) => part.length > 0 && part !== \".\" && part !== \"..\");\n}",
        "sha256": "62dd9e376e3656cee2fe2bfae51f39174d15e9bebab7fbc5f1185338d089a9fd",
        "startLine": 2404,
        "endLine": 2406
      },
      {
        "name": "sourceAdmissionGitRecords",
        "text": "function sourceAdmissionGitRecords(bytes: Uint8Array, label: string): readonly string[] {\n  if (bytes.length === 0) return [];\n  if (bytes[bytes.length - 1] !== 0) throw new Error(`${label} is missing its terminal NUL`);\n  const rows = new TextDecoder(\"utf-8\", { fatal: true }).decode(bytes).slice(0, -1).split(\"\\0\");\n  if (rows.some((row) => !row)) throw new Error(`${label} contains an empty record`);\n  return rows;\n}",
        "sha256": "944605eb2251fa68d86da1d4d41bb9d3d5e93bf306755dd004542b5eda73b5bf",
        "startLine": 2739,
        "endLine": 2745
      },
      {
        "name": "sourceAdmissionGitExclusions",
        "text": "function sourceAdmissionGitExclusions(pathspec: TaxonomyScopedGitPathspec): readonly string[] {\n  return [...pathspec.exclusionPathspecs, \":(exclude,icase,glob)**/compose\", \":(exclude,icase,glob)**/compose/**\"];\n}",
        "sha256": "57ac49ba06eb202a05278ebc85190ba9f6970c116d1ede350af8a4e85e433755",
        "startLine": 2747,
        "endLine": 2749
      },
      {
        "name": "sourceAdmissionUntrackedPaths",
        "text": "function sourceAdmissionUntrackedPaths(repoRoot: string, pathspec: TaxonomyScopedGitPathspec, taxonomy: LoadedTaxonomy): readonly string[] {\n  const literal = (path: string): string => path.replace(/[\\\\*?\\[\\]#! ]/gu, \"\\\\$&\");\n  const exclusions = taxonomy.exclusions.map((entry) => `--exclude=/${literal(entry.path)}`);\n  const bytes = execFileSync(\"git\", [\"ls-files\", \"--others\", \"--exclude-standard\", \"--exclude=[cC][oO][mM][pP][oO][sS][eE]\", ...exclusions, \"-z\", \"--\", pathspec.positivePathspec, ...sourceAdmissionGitExclusions(pathspec)], { cwd: repoRoot, encoding: \"buffer\", maxBuffer: 256 * 1024 * 1024 });\n  const rows = sourceAdmissionGitRecords(bytes, \"Git untracked output\");\n  if (rows.some((path) => !sourceAdmissionSafePath(path))) throw new Error(\"Git untracked output has an invalid source path\");\n  return [...rows].sort(sourceAdmissionByteCompare);\n}",
        "sha256": "6fbd759504d75e8a30eea893497999eba6b1f045294d36e25c3c56a10971a14b",
        "startLine": 2761,
        "endLine": 2768
      },
      {
        "name": "sourceAdmissionByteCompare",
        "text": "const sourceAdmissionByteCompare = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));",
        "sha256": "a8ea553a484f18c49e1a07ebbc9d7825f6a323f661e43445d0192a77b7b3f6b1",
        "startLine": 2402,
        "endLine": 2402
      }
    ],
    "helperSourceSha256": "d37fdcab3c16a1645889dcb1a3dc9357aef8605848088986b96d27b6d9461df1",
    "transpiledSha256": "b50b821ac828d107d9aefc0d2c10e94d4859705f49bb2a552ec72b4483edc0cb"
  },
  "taxonomyEvidence": {
    "loadedContentHash": "84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce",
    "capturedContentHash": "84455e5e4cd458bcf95ae613d6af909d61ce7805b10a03592d7b29320afcd0ce",
    "actualExclusions": [
      {
        "id": "compose",
        "path": "compose"
      },
      {
        "id": "temp-compose",
        "path": "temp/compose"
      }
    ],
    "inputBytesMatch": true
  },
  "pathspec": {
    "normalizedScope": null,
    "conservativePrefix": ".",
    "positivePathspec": ".",
    "exclusionPathspecs": [
      ":(exclude,top,literal)compose",
      ":(exclude,top,literal)temp/compose"
    ]
  },
  "gitCalls": 1,
  "git": {
    "command": "git",
    "args": [
      "ls-files",
      "--others",
      "--exclude-standard",
      "--exclude=[cC][oO][mM][pP][oO][sS][eE]",
      "--exclude=/compose",
      "--exclude=/temp/compose",
      "-z",
      "--",
      ".",
      ":(exclude,top,literal)compose",
      ":(exclude,top,literal)temp/compose",
      ":(exclude,icase,glob)**/compose",
      ":(exclude,icase,glob)**/compose/**"
    ],
    "originalOptions": {
      "cwd": "/Users/ueli/Documents/semio",
      "encoding": "buffer",
      "maxBuffer": 268435456
    },
    "effectiveOptions": {
      "cwd": "/Users/ueli/Documents/semio",
      "encoding": "buffer",
      "maxBuffer": 268435456,
      "timeout": 30000,
      "killSignal": "SIGKILL"
    },
    "timeoutMs": 30000,
    "startedAt": "2026-08-27T22:56:14.411Z",
    "status": "returned",
    "stderrOnFailureBase64": null,
    "exitCode": 0,
    "signal": null,
    "durationMs": 4405.140875,
    "completedAt": "2026-08-27T22:56:18.816Z",
    "stdoutBytes": 53426,
    "stdoutSha256": "fde3be9741e4432e86de0e782e592035eaab59574c9ee26b33fc115c924d61c9"
  },
  "helperResult": {
    "status": "threw",
    "count": null,
    "error": {
      "name": "Error",
      "message": "Git untracked output has an invalid source path",
      "stack": "Error: Git untracked output has an invalid source path\n    at sourceAdmissionUntrackedPaths (file:///Users/ueli/Documents/semio/.%F0%9F%A7%ACsemio/%F0%9F%A6%91%EF%B8%8Frepo/%F0%9F%8E%AB%EF%B8%8Ftickets/%F0%9F%8E%86%EF%B8%8F26/%F0%9F%8C%99%EF%B8%8F08/%E2%98%80%EF%B8%8F12/SEMANTIC-MUTATIONS-OVERHAUL/%F0%9F%93%93%EF%B8%8Fadmission-untracked-diagnostic-55/%F0%9F%93%9C%EF%B8%8Fscript.ts:25:24)\n    at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/📜️script.ts:166:32"
    }
  },
  "framing": {
    "accepted": true,
    "recordCount": 320,
    "error": null
  },
  "rawOutput": {
    "path": "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-untracked-diagnostic-55/🧫️run-SOCd6O/🔣️git-stdout.bin",
    "bytes": 53426,
    "sha256": "fde3be9741e4432e86de0e782e592035eaab59574c9ee26b33fc115c924d61c9",
    "base64": "LvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLWN1cnJlbnQtcm9zdGVyLTU0LWN1cnJlbnQtMjAyNi0wOC0yN1QyMi00Ny0wMS01MzBaLfCfp6vvuI9ydW4tUVNYaDVjLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1jdXJyZW50LXJvc3Rlci01NC1vdXRjb21lLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1jdXJyZW50LXJvc3Rlci01NC1wbGFuLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1jdXJyZW50LXJvc3Rlci01NC12YWxpZGF0ZS0yMDI2LTA4LTI3VDIyLTQ1LTUxLTM3Mlot8J+nq++4j3NjaGVtYS14SktyeHcubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLWN1cnJlbnQtcm9zdGVyLTU0L/Cfk5zvuI9zY3JpcHQudHMALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLWN1cnJlbnQtcm9zdGVyLTU0L/CflKPvuI92ZWN0b3JzLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLWN1cnJlbnQtcm9zdGVyLTU0L/Cfp6vvuI9ydW4tUVNYaDVjL/Cfk5PvuI9yZWNlaXB0Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1jdXJyZW50LXJvc3Rlci01NC/wn6er77iPcnVuLVFTWGg1Yy/wn5Sj77iPaW5wdXQuanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tY3VycmVudC1yb3N0ZXItNTQv8J+nq++4j3J1bi1RU1hoNWMv8J+Uo++4j3JlY2VpcHQuanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tY3VycmVudC1yb3N0ZXItNTQv8J+nq++4j3NjaGVtYS14SktyeHcv8J+Tk++4j3JlY2VpcHQubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLWN1cnJlbnQtcm9zdGVyLTU0L/Cfp6vvuI9zY2hlbWEteEpLcnh3L/CflKPvuI9yZWNlaXB0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLWN1cnJlbnQtcm9zdGVyLTU0L/Cfp6zvuI9zY2hlbWEv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1waHlzaWNhbC01My3wn6er77iPcnVuLUN0Z0pZZC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tcGh5c2ljYWwtNTMt8J+nq++4j3J1bi1TamM0Z0UubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXBoeXNpY2FsLTUzLfCfp6vvuI9ydW4tbzIxUjJyLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1waHlzaWNhbC1pbnRlZ3JhdGlvbi01My/wn5Oc77iPc2NyaXB0LnRzAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1waHlzaWNhbC1pbnRlZ3JhdGlvbi01My/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1waHlzaWNhbC1pbnRlZ3JhdGlvbi01My/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXBoeXNpY2FsLWludGVncmF0aW9uLTUzL/Cfp6vvuI9ydW4tQ3RnSllkL/Cfk53vuI8ubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXBoeXNpY2FsLWludGVncmF0aW9uLTUzL/Cfp6vvuI9ydW4tQ3RnSllkL/CflKPvuI9yZWNlaXB0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXBoeXNpY2FsLWludGVncmF0aW9uLTUzL/Cfp6vvuI9ydW4tQ3RnSllkL/Cfp6rvuI9maXh0dXJlLwAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tcGh5c2ljYWwtaW50ZWdyYXRpb24tNTMv8J+nq++4j3J1bi1TamM0Z0Uv8J+Tne+4jy5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tcGh5c2ljYWwtaW50ZWdyYXRpb24tNTMv8J+nq++4j3J1bi1TamM0Z0Uv8J+Uo++4j3JlY2VpcHQuanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tcGh5c2ljYWwtaW50ZWdyYXRpb24tNTMv8J+nq++4j3J1bi1TamM0Z0Uv8J+nqu+4j2ZpeHR1cmUvAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1waHlzaWNhbC1pbnRlZ3JhdGlvbi01My/wn6er77iPcnVuLW8yMVIyci/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1waHlzaWNhbC1pbnRlZ3JhdGlvbi01My/wn6er77iPcnVuLW8yMVIyci/wn5Sj77iPcmVjZWlwdC5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1waHlzaWNhbC1pbnRlZ3JhdGlvbi01My/wn6er77iPcnVuLW8yMVIyci/wn6eq77iPZml4dHVyZS8ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXBoeXNpY2FsLWludGVncmF0aW9uLTUzL/Cfp6zvuI9zY2hlbWEv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi1zb3VyY2UtcmVsZWFzZS01NC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdW50cmFja2VkLWRpYWdub3N0aWMtNTUv8J+TnO+4j3NjcmlwdC50cwAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdW50cmFja2VkLWRpYWdub3N0aWMtNTUv8J+nq++4j3J1bi1TT0NkNk8v8J+nrO+4j0Quc291cmNlLnR4dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdW50cmFja2VkLWRpYWdub3N0aWMtNTUv8J+nq++4j3J1bi1TT0NkNk8v8J+nrO+4j04uc291cmNlLnR4dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdW50cmFja2VkLWRpYWdub3N0aWMtNTUv8J+nq++4j3J1bi1TT0NkNk8v8J+nrO+4j2NvbnRyb2xsZXIuc291cmNlLnR4dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdW50cmFja2VkLWRpYWdub3N0aWMtNTUv8J+nq++4j3J1bi1TT0NkNk8v8J+nrO+4j3RheG9ub215LnNvdXJjZS50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My1pby0yMDI2LTA4LTI3VDIyLTM1LTEyLTE1Mlot8J+nq++4j3J1bi0wTnJieGgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My1pby0yMDI2LTA4LTI3VDIyLTM3LTMxLTExM1ot8J+nq++4j3J1bi1lMnRKVjEubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My1pby0yMDI2LTA4LTI3VDIyLTM4LTE1LTUxMlot8J+nq++4j3J1bi05UlVVdjYubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My1zb3VyY2UtMjAyNi0wOC0yN1QyMi0yMy0wOC0wNDZaLfCfp6vvuI9ydW4tSVIybW90Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j2FkbWlzc2lvbi12ZXJpZmljYXRpb24tNTMtc291cmNlLTIwMjYtMDgtMjdUMjItMzktNDAtMzgzWi3wn6er77iPcnVuLXZKV2Z6Ri5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdmVyaWZpY2F0aW9uLTUzL/Cfk5zvuI9zY3JpcHQudHMALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My/wn6er77iPcnVuLTBOcmJ4aC/wn5OT77iPcmVjZWlwdC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdmVyaWZpY2F0aW9uLTUzL/Cfp6vvuI9ydW4tME5yYnhoL/CflKPvuI9yZWNlaXB0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My/wn6er77iPcnVuLTlSVVV2Ni/wn5OT77iPcmVjZWlwdC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdmVyaWZpY2F0aW9uLTUzL/Cfp6vvuI9ydW4tOVJVVXY2L/CflKPvuI9yZWNlaXB0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My/wn6er77iPcnVuLUlSMm1vdC/wn5OT77iPcmVjZWlwdC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdmVyaWZpY2F0aW9uLTUzL/Cfp6vvuI9ydW4tSVIybW90L/CflKPvuI9yZWNlaXB0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My/wn6er77iPcnVuLWUydEpWMS/wn5OT77iPcmVjZWlwdC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdmVyaWZpY2F0aW9uLTUzL/Cfp6vvuI9ydW4tZTJ0SlYxL/CflKPvuI9yZWNlaXB0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPYWRtaXNzaW9uLXZlcmlmaWNhdGlvbi01My/wn6er77iPcnVuLXZKV2Z6Ri/wn5OT77iPcmVjZWlwdC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI9hZG1pc3Npb24tdmVyaWZpY2F0aW9uLTUzL/Cfp6vvuI9ydW4tdkpXZnpGL/CflKPvuI9yZWNlaXB0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPbXV0YXRpb24tY2Vuc3VzLWFuZC1uYXRpdmUtcmV2aWV3LTU1Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+Tk++4j211dGF0aW9uLXN0cnVjdHVyYWwtc291cmNlLXZpZXctNTItcHJlcGFyYXRpb24ubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPcGx1Z2luLWRlY2xhcmF0aW9ucy1pbXBvcnQtNTMubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPc291cmNlLWFkbWlzc2lvbi1jYW5vbmljYWwtbW91bnQtNTMubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn5OT77iPdHhuLWFjdGlvbi1qb2ItY2xvc2UtYnVkZ2V0LXJlZC01NC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfk5PvuI90eG4tY29tbWFuZC1jbG9zZS1uYXRpdmUtNTUtcmV2aWV3Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j211dGF0aW9uLXN0cnVjdHVyYWwtc291cmNlLXZpZXctNTIv8J+nq++4j3J1bnMvZmFsbGJhY2stcmVkL3Jlc3VsdC5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j211dGF0aW9uLXN0cnVjdHVyYWwtc291cmNlLXZpZXctNTIv8J+nq++4j3J1bnMvdHlwZS1kaWFnbm9zdGljcy1lYWExNTE4Ni1mOTFkLTQ1OWEtODYxMy1lZjhiNmVlYWVmNDcvcmVzdWx0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn6eq77iPbXV0YXRpb24tc3RydWN0dXJhbC1zb3VyY2Utdmlldy01Mi/wn6er77iPcnVucy90eXBlLWRpYWdub3N0aWNzLWVhYTcyMTllLWQyZTUtNDA2Yy1iMDIwLTMyYTFiZTgwOTRmZS9yZXN1bHQuanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI9tdXRhdGlvbi1zdHJ1Y3R1cmFsLXNvdXJjZS12aWV3LTUyL/Cfp6vvuI9ydW5zL3ZpZXctZ3JlZW4tMTBkNDA1ZWUtZDgzZC00NzIwLThiZmYtM2Q1YmE0MGYzMzNhL3Jlc3VsdC5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j211dGF0aW9uLXN0cnVjdHVyYWwtc291cmNlLXZpZXctNTIv8J+nq++4j3J1bnMvdmlldy1ncmVlbi0xZGM1Nzk2MS0zNjQyLTRjOTgtOGY2Yy0zMzAzM2ZlZDNjYmYvcmVzdWx0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn6eq77iPbXV0YXRpb24tc3RydWN0dXJhbC1zb3VyY2Utdmlldy01Mi/wn6er77iPcnVucy92aWV3LWdyZWVuLTY2NzhjYzA4LTQ5MjMtNGUyNi05YTQ3LWU2M2UwMWFlMDNmNy9yZXN1bHQuanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI9tdXRhdGlvbi1zdHJ1Y3R1cmFsLXNvdXJjZS12aWV3LTUyL/Cfp6vvuI9ydW5zL3ZpZXctZ3JlZW4tYzc2M2VhMTgtYjlkMi00NjliLTlmOGYtYmE1NmYxYWE3ZDM1L3Jlc3VsdC5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j211dGF0aW9uLXN0cnVjdHVyYWwtc291cmNlLXZpZXctNTIv8J+nq++4j3J1bnMvdmlldy1ncmVlbi1jYWQ4OTMyMS01ZTE0LTQ2YjItYjFlMC04YjZiNDA1MTRkOWUvcmVzdWx0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn6eq77iPbXV0YXRpb24tc3RydWN0dXJhbC1zb3VyY2Utdmlldy01Mi/wn6er77iPcnVucy92aWV3LWdyZWVuLWVhMTM2ODlmLTJkZGEtNDk4ZC04MDhmLWY4N2FmYTczMTFhOC9yZXN1bHQuanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI9tdXRhdGlvbi1zdHJ1Y3R1cmFsLXNvdXJjZS12aWV3LTUyL/Cfp6vvuI9ydW5zL3ZpZXctZ3JlZW4tZmQwYjMxMjQtMjg5MC00ZmQxLTkwNWUtOWY4OTEyYzIzZDFmL3Jlc3VsdC5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j211dGF0aW9uLXN0cnVjdHVyYWwtc291cmNlLXZpZXctNTIv8J+nq++4j3J1bnMvdmlldy1ncmVlbi9yZXN1bHQuanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI9zb3VyY2UtYWRtaXNzaW9uLWNhbm9uaWNhbC1tb3VudC01My/wn5Oc77iPc2NyaXB0LnRzAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j3NvdXJjZS1hZG1pc3Npb24tY2Fub25pY2FsLW1vdW50LTUzL/Cfp6vvuI9ydW4tNVlyUXhlL/CflKPvuI9yZXN1bHQuanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI90eG4tYWN0aW9uLWpvYi1jbG9zZS01NC/wn5Oc77iPc2NyaXB0LnRzAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j3R4bi1hY3Rpb24tam9iLWNsb3NlLTU0L/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI90eG4tYWN0aW9uLWpvYi1jbG9zZS01NC/wn6aA77iPbmF0aXZlLXJlZC5ycwAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI90eG4tYWN0aW9uLWpvYi1jbG9zZS01NC/wn6es77iPc2NoZW1hL/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI90eG4tY29tbWFuZC1jbG9zZS1uYXRpdmUtNTUv8J+TnO+4j3NjcmlwdC50cwAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTIvU0VNQU5USUMtTVVUQVRJT05TLU9WRVJIQVVML/Cfp6rvuI90eG4tY29tbWFuZC1jbG9zZS1uYXRpdmUtNTUv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j3R4bi1jb21tYW5kLWNsb3NlLW5hdGl2ZS01NS/wn6aA77iPLnJzAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j3R4bi1jb21tYW5kLWNsb3NlLW5hdGl2ZS01NS/wn6er77iPcnVuLTBwaHVOQS/wn5OT77iPcnVuLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xMi9TRU1BTlRJQy1NVVRBVElPTlMtT1ZFUkhBVUwv8J+nqu+4j3R4bi1jb21tYW5kLWNsb3NlLW5hdGl2ZS01NS/wn6er77iPcnVuLTBwaHVOQS/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzEyL1NFTUFOVElDLU1VVEFUSU9OUy1PVkVSSEFVTC/wn6eq77iPdHhuLWNvbW1hbmQtY2xvc2UtbmF0aXZlLTU1L/Cfp6zvuI9zY2hlbWEv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2Nvb3JkaW5hdG9yLXByZXNlcnZhdGlvbi0yMDI2LTA4LTI4L/Cfk5zvuI9zY3JpcHQudHMALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPY29vcmRpbmF0b3ItcHJlc2VydmF0aW9uLTIwMjYtMDgtMjgv8J+Tne+4jy5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9sYXVuY2gtcm93LWFkbWlzc2lvbi0yMDI2LTA4LTI4L/Cfk5zvuI9zY3JpcHQudHMALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2xhdW5jaC1yb3ctYWRtaXNzaW9uLTIwMjYtMDgtMjgv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2xhdW5jaC1yb3ctYWRtaXNzaW9uLTIwMjYtMDgtMjgv8J+nqu+4j3J1bnMv8J+Ulu+4jzBhODE1YjBjLTU5MmUtNDRkMC05MmFlLWE0OWZkMzI5OTljZC/wn5SX77iPaW5wdXQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn6eq77iPcnVucy/wn5SW77iPMGE4MTViMGMtNTkyZS00NGQwLTkyYWUtYTQ5ZmQzMjk5OWNkL/CflJfvuI9wYXJlbnQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn6eq77iPcnVucy/wn5SW77iPMGE4MTViMGMtNTkyZS00NGQwLTkyYWUtYTQ5ZmQzMjk5OWNkL/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9sYXVuY2gtcm93LWFkbWlzc2lvbi0yMDI2LTA4LTI4L/Cfp6rvuI9ydW5zL/CflJbvuI8wYzJkODg1My01OGVlLTQwNmUtODc1NS1kNmMxOTY2MGM4ZTQv8J+Ul++4j2lucHV0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2xhdW5jaC1yb3ctYWRtaXNzaW9uLTIwMjYtMDgtMjgv8J+nqu+4j3J1bnMv8J+Ulu+4jzBjMmQ4ODUzLTU4ZWUtNDA2ZS04NzU1LWQ2YzE5NjYwYzhlNC/wn5SX77iPcGFyZW50AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2xhdW5jaC1yb3ctYWRtaXNzaW9uLTIwMjYtMDgtMjgv8J+nqu+4j3J1bnMv8J+Ulu+4jzBjMmQ4ODUzLTU4ZWUtNDA2ZS04NzU1LWQ2YzE5NjYwYzhlNC/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn6eq77iPcnVucy/wn5SW77iPMjczNjM2NGQtYzUyNC00ZGFmLWI3NTgtMzI0MTNiODJiN2M1L/CflJfvuI9pbnB1dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9sYXVuY2gtcm93LWFkbWlzc2lvbi0yMDI2LTA4LTI4L/Cfp6rvuI9ydW5zL/CflJbvuI8yNzM2MzY0ZC1jNTI0LTRkYWYtYjc1OC0zMjQxM2I4MmI3YzUv8J+Ul++4j3BhcmVudAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9sYXVuY2gtcm93LWFkbWlzc2lvbi0yMDI2LTA4LTI4L/Cfp6rvuI9ydW5zL/CflJbvuI8yNzM2MzY0ZC1jNTI0LTRkYWYtYjc1OC0zMjQxM2I4MmI3YzUv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2xhdW5jaC1yb3ctYWRtaXNzaW9uLTIwMjYtMDgtMjgv8J+nqu+4j3J1bnMv8J+Ulu+4jzMwYjAzNTUxLWZhOTctNGEwZC05N2I3LWNiY2MxZjRhMDg3Zi/wn5SX77iPaW5wdXQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn6eq77iPcnVucy/wn5SW77iPMzBiMDM1NTEtZmE5Ny00YTBkLTk3YjctY2JjYzFmNGEwODdmL/CflJfvuI9wYXJlbnQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn6eq77iPcnVucy/wn5SW77iPMzBiMDM1NTEtZmE5Ny00YTBkLTk3YjctY2JjYzFmNGEwODdmL/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9sYXVuY2gtcm93LWFkbWlzc2lvbi0yMDI2LTA4LTI4L/Cfp6rvuI9ydW5zL/CflJbvuI84NTA0ZWM3Ni1kOWJhLTQyNTUtODdiOC02ZDJkMDM0ODkzNTkv8J+Ul++4j2lucHV0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2xhdW5jaC1yb3ctYWRtaXNzaW9uLTIwMjYtMDgtMjgv8J+nqu+4j3J1bnMv8J+Ulu+4jzg1MDRlYzc2LWQ5YmEtNDI1NS04N2I4LTZkMmQwMzQ4OTM1OS/wn5SX77iPcGFyZW50AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2xhdW5jaC1yb3ctYWRtaXNzaW9uLTIwMjYtMDgtMjgv8J+nqu+4j3J1bnMv8J+Ulu+4jzg1MDRlYzc2LWQ5YmEtNDI1NS04N2I4LTZkMmQwMzQ4OTM1OS/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn6eq77iPcnVucy/wn5SW77iPY2Y3NjU0ZDEtNzUzMC00NmM2LWJhN2UtOWRkZDFlM2RiMjk4L/CflJfvuI9pbnB1dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9sYXVuY2gtcm93LWFkbWlzc2lvbi0yMDI2LTA4LTI4L/Cfp6rvuI9ydW5zL/CflJbvuI9jZjc2NTRkMS03NTMwLTQ2YzYtYmE3ZS05ZGRkMWUzZGIyOTgv8J+Ul++4j3BhcmVudAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9sYXVuY2gtcm93LWFkbWlzc2lvbi0yMDI2LTA4LTI4L/Cfp6rvuI9ydW5zL/CflJbvuI9jZjc2NTRkMS03NTMwLTQ2YzYtYmE3ZS05ZGRkMWUzZGIyOTgv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j2xhdW5jaC1yb3ctYWRtaXNzaW9uLTIwMjYtMDgtMjgv8J+nqu+4j3J1bnMv8J+Ulu+4j2UyZTU1NjJkLTNiMWYtNGRkMi1hY2ViLWI4NjFhODRmNjk1NS/wn5SX77iPaW5wdXQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn6eq77iPcnVucy/wn5SW77iPZTJlNTU2MmQtM2IxZi00ZGQyLWFjZWItYjg2MWE4NGY2OTU1L/CflJfvuI9wYXJlbnQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbGF1bmNoLXJvdy1hZG1pc3Npb24tMjAyNi0wOC0yOC/wn6eq77iPcnVucy/wn5SW77iPZTJlNTU2MmQtM2IxZi00ZGQyLWFjZWItYjg2MWE4NGY2OTU1L/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9sYXVuY2gtcm93LWFkbWlzc2lvbi0yMDI2LTA4LTI4L/Cfp6zvuI9zY2hlbWEv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn5OT77iPcmVzdWx0cy/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn5Oc77iPc2NyaXB0LnRzAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24tY3VycmVudC1zb3VyY2UtcmVwbGF5L/Cfp77vuI9ydW5zL/CflJbvuI84NDk4ODRjNS03YzU3LTQxYWEtOWM1MC0yOTU3MjBkZDdlMDMv8J+Tk++4j2J1bi/wn5OT77iPaW5wdXQv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn6e+77iPcnVucy/wn5SW77iPODQ5ODg0YzUtN2M1Ny00MWFhLTljNTAtMjk1NzIwZGQ3ZTAzL/Cfk5PvuI9idW4v8J+Tk++4j291dHB1dC/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24tY3VycmVudC1zb3VyY2UtcmVwbGF5L/Cfp77vuI9ydW5zL/CflJbvuI84NDk4ODRjNS03YzU3LTQxYWEtOWM1MC0yOTU3MjBkZDdlMDMv8J+Tk++4j2J1bi/wn5OT77iPc3RkZXJyL/CflKTvuI8udHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn6e+77iPcnVucy/wn5SW77iPODQ5ODg0YzUtN2M1Ny00MWFhLTljNTAtMjk1NzIwZGQ3ZTAzL/Cfk5PvuI9idW4v8J+Tk++4j3N0ZG91dC/wn5Sk77iPLnR4dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1jdXJyZW50LXNvdXJjZS1yZXBsYXkv8J+nvu+4j3J1bnMv8J+Ulu+4jzg0OTg4NGM1LTdjNTctNDFhYS05YzUwLTI5NTcyMGRkN2UwMy/wn5OT77iPY2FwdHVyZS/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24tY3VycmVudC1zb3VyY2UtcmVwbGF5L/Cfp77vuI9ydW5zL/CflJbvuI84NDk4ODRjNS03YzU3LTQxYWEtOWM1MC0yOTU3MjBkZDdlMDMv8J+Tk++4j254L/CflKTvuI8udHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn6e+77iPcnVucy/wn5SW77iPODQ5ODg0YzUtN2M1Ny00MWFhLTljNTAtMjk1NzIwZGQ3ZTAzL/Cfk5PvuI9yZWFkYmFjay/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24tY3VycmVudC1zb3VyY2UtcmVwbGF5L/Cfp77vuI9ydW5zL/CflJbvuI84NDk4ODRjNS03YzU3LTQxYWEtOWM1MC0yOTU3MjBkZDdlMDMv8J+Tk++4j3R5cGVzY3JpcHQv8J+Tk++4j2lucHV0L/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1jdXJyZW50LXNvdXJjZS1yZXBsYXkv8J+nvu+4j3J1bnMv8J+Ulu+4jzg0OTg4NGM1LTdjNTctNDFhYS05YzUwLTI5NTcyMGRkN2UwMy/wn5OT77iPdHlwZXNjcmlwdC/wn5OT77iPb3V0cHV0L/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1jdXJyZW50LXNvdXJjZS1yZXBsYXkv8J+nvu+4j3J1bnMv8J+Ulu+4jzg0OTg4NGM1LTdjNTctNDFhYS05YzUwLTI5NTcyMGRkN2UwMy/wn5OT77iPdHlwZXNjcmlwdC/wn5OT77iPc3RkZXJyL/CflKTvuI8udHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn6e+77iPcnVucy/wn5SW77iPODQ5ODg0YzUtN2M1Ny00MWFhLTljNTAtMjk1NzIwZGQ3ZTAzL/Cfk5PvuI90eXBlc2NyaXB0L/Cfk5PvuI9zdGRvdXQv8J+UpO+4jy50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24tY3VycmVudC1zb3VyY2UtcmVwbGF5L/Cfp77vuI9ydW5zL/CflJbvuI84NDk4ODRjNS03YzU3LTQxYWEtOWM1MC0yOTU3MjBkZDdlMDMv8J+TnO+4j3NjcmlwdC50cwAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1jdXJyZW50LXNvdXJjZS1yZXBsYXkv8J+nvu+4j3J1bnMv8J+Ulu+4jzg0OTg4NGM1LTdjNTctNDFhYS05YzUwLTI5NTcyMGRkN2UwMy/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWN1cnJlbnQtc291cmNlLXJlcGxheS/wn6e+77iPcnVucy/wn5SW77iPODQ5ODg0YzUtN2M1Ny00MWFhLTljNTAtMjk1NzIwZGQ3ZTAzL/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtbGluay1zb3VyY2UtcmV2aWV3L/Cfk53vuI8ubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLWxpbmstc3ludGhldGljLWFic2VuY2Uv8J+Tne+4jy5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtcmVmZXJlbmNlLXJlZ2lzdHJhdGlvbi/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWlubGluZS1yZWZlcmVuY2UtcmVnaXN0cmF0aW9uL/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtcmVmZXJlbmNlLXJlZ2lzdHJhdGlvbi/wn6eq77iPbGF1bmNoLXByZWZsaWdodC1yMi5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWlubGluZS1yZWZlcmVuY2UtcmVnaXN0cmF0aW9uL/Cfp6rvuI9sYXVuY2gtcHJlZmxpZ2h0Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZS1yZWdpc3RyYXRpb24v8J+nqu+4j2xhdW5jaC1wcmV2aWV3Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZS10ZXN0cy/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWlubGluZS1yZWZlcmVuY2UtdGVzdHMv8J+nvu+4j3J1bnMv8J+Ulu+4j0Rlb0pFaC/wn5Oc77iPc2NyaXB0LnRzAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWlubGluZS1yZWZlcmVuY2UtdGVzdHMv8J+nvu+4j3J1bnMv8J+Ulu+4j0Rlb0pFaC/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWlubGluZS1yZWZlcmVuY2UtdGVzdHMv8J+nvu+4j3J1bnMv8J+Ulu+4j0Rlb0pFaC/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZS10ZXN0cy/wn6e+77iPcnVucy/wn5SW77iPTTRaUWR1L/Cfk5zvuI9zY3JpcHQudHMALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZS10ZXN0cy/wn6e+77iPcnVucy/wn5SW77iPTTRaUWR1L/Cfk53vuI8ubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZS10ZXN0cy/wn6e+77iPcnVucy/wn5SW77iPTTRaUWR1L/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtcmVmZXJlbmNlLXRlc3RzL/Cfp77vuI9ydW5zL/CflJbvuI9UQWxXNWwv8J+TnO+4j3NjcmlwdC50cwAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtcmVmZXJlbmNlLXRlc3RzL/Cfp77vuI9ydW5zL/CflJbvuI9UQWxXNWwv8J+Tne+4jy5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtcmVmZXJlbmNlLXRlc3RzL/Cfp77vuI9ydW5zL/CflJbvuI9UQWxXNWwv8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWlubGluZS1yZWZlcmVuY2UtdGVzdHMv8J+nvu+4j3J1bnMv8J+Ulu+4j1gwdXJXSC/wn5Oc77iPc2NyaXB0LnRzAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWlubGluZS1yZWZlcmVuY2UtdGVzdHMv8J+nvu+4j3J1bnMv8J+Ulu+4j1gwdXJXSC/wn5Od77iPLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j21hcmtkb3duLWlubGluZS1yZWZlcmVuY2UtdGVzdHMv8J+nvu+4j3J1bnMv8J+Ulu+4j1gwdXJXSC/wn5Sj77iPLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZS10ZXN0cy/wn6e+77iPcnVucy/wn5SW77iPdmZRRzl0L/Cfk5zvuI9zY3JpcHQudHMALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZS10ZXN0cy/wn6e+77iPcnVucy/wn5SW77iPdmZRRzl0L/Cfk53vuI8ubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZS10ZXN0cy/wn6e+77iPcnVucy/wn5SW77iPdmZRRzl0L/CflKPvuI8uanNvbgAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtcmVmZXJlbmNlLXRlc3RzL/Cfp77vuI9ydW5zL/CflJbvuI96YkM1R1Av8J+TnO+4j3NjcmlwdC50cwAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtcmVmZXJlbmNlLXRlc3RzL/Cfp77vuI9ydW5zL/CflJbvuI96YkM1R1Av8J+Tne+4jy5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRU5ELVRPLUVORC1UQVhPTk9NWS1OT1JNQUxJWkFUSU9OL/Cfk5PvuI9tYXJrZG93bi1pbmxpbmUtcmVmZXJlbmNlLXRlc3RzL/Cfp77vuI9ydW5zL/CflJbvuI96YkM1R1Av8J+Uo++4jy5qc29uAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9FTkQtVE8tRU5ELVRBWE9OT01ZLU5PUk1BTElaQVRJT04v8J+Tk++4j3JlYWRtZS1jdXJyZW50LXBsYW4tcmVhZGluZXNzL/Cfk53vuI8ubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0VORC1UTy1FTkQtVEFYT05PTVktTk9STUFMSVpBVElPTi/wn5OT77iPdWktaG9zdC1wYWNrYWdlLW1ldGFkYXRhL/Cfk53vuI8ubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzE3L0ZJWC1ERU1PTlNUUkFUT1ItRU5ELVRPLUVORC1CT09ULUhBTkcv8J+Tk++4j2NhcHR1cmVkLXJldHVybi1jb25zdHJ1Y3Rvci1vd25lcnNoaXAtMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMTcvRklYLURFTU9OU1RSQVRPUi1FTkQtVE8tRU5ELUJPT1QtSEFORy/wn5OT77iPcG9vbC1hZG1pc3Npb24tY2VsbC1pbnRlZ3JhdGlvbi0yMDI2LTA4LTI4Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8xNy9GSVgtREVNT05TVFJBVE9SLUVORC1UTy1FTkQtQk9PVC1IQU5HL/Cfk5PvuI9yZXRhaW5lZC1jb250cm9sbGVyLXJvb3QtY29udHJhY3QtMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tiu+4j3BsdWdpbi1jb25zdC1vd25lci1zZWxlY3RlZC1wYXRocy1yMS0yMDI2LTA4LTI4Lmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9hY3Rpb25idXMtc2hvcnQtY2xvc2UtY29uc2VydmF0aW9uLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9hY3Rpb25idXMtc2hvcnQtY2xvc2UtcjItbmF0aXZlLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9jb21tb24ta2VybmVsLWZ1bGwtcjItbmF0aXZlLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9jb21tb24ta2VybmVsLXIyLXNlbGVjdGVkLWlucHV0cy0yMDI2LTA4LTI4Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn5OT77iPY29vcmRpbmF0b3ItbmF0aXZlLWludGVncmF0aW9uLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9jb29yZGluYXRvci1wb2xsLWNvbXBvc2l0aW9uLXIxLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9jb29yZGluYXRvci1yZXNpZGVudC1yNC1hbmQtcXVhcmFudGluZS1yZWQtMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tk++4j2Nvb3JkaW5hdG9yLXJlc2lkZW50LXI1LWFuZC1xdWFyYW50aW5lLWdyZWVuLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9jb29yZGluYXRvci1yZXN0YXJ0LXIxLXIzLW5hdGl2ZS1yZXZpZXctMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tk++4j2Nvb3JkaW5hdG9yLXJldGFpbmVkLWNvbnRyb2xsZXItcjEtMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tk++4j2Nvb3JkaW5hdG9yLXVpLXBhcmVudC1zbG90LXJldmlldy0yMDI2LTA4LTI4Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn5OT77iPY29vcmRpbmF0b3Itd2dwdS1pbnB1dC1hZG1pc3Npb24tcmV2aWV3LTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9mb3JlaWduLXN0ZGlvLWNvbXBpbGVyLW9ic2VydmF0aW9uLXIyLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9wbHVnaW4tY2hlY2twb2ludC1yNC1zZWxlY3RlZC1pbnB1dHMtMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tk++4j3BsdWdpbi1jaGVja3BvaW50LXI2LXNlbGVjdGVkLWlucHV0cy0yMDI2LTA4LTI4Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn5OT77iPcGx1Z2luLWNoZWNrcG9pbnQtdGFpbC1yNC1uYXRpdmUtcmVkLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9wbHVnaW4tY29uc3Qtb3duZXItcjEtbmF0aXZlLXJlZC0yMDI2LTA4LTI4Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn5OT77iPcGx1Z2luLWNvbnN0LW93bmVyLXIxLXNlbGVjdGVkLWlucHV0cy0yMDI2LTA4LTI4Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn5OT77iPcGx1Z2luLWRlY2xhcmF0aW9ucy1pbXBvcnQtcmVwYWlyLm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn5OT77iPcGx1Z2luLXJlc3RhcnQtcjItc2VsZWN0ZWQtaW5wdXRzLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9wbHVnaW4tcmVzdGFydC1yMy1zZWxlY3RlZC1pbnB1dHMtMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tk++4j3BsdWdpbi1yZXN0YXJ0LXJlZ3Jlc3Npb24tcjUtbmF0aXZlLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9wbHVnaW4tcmVzdGFydC10d28tcjItbmF0aXZlLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI9wbHVnaW4tcmVzdGFydC10d28tcjMtbmF0aXZlLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfk5PvuI90dXRvcmlhbC1kb2N1bWVudC1maWVsZC1uYXRpdmUtMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tk++4j3dncHUtZXZlbnQtbWV0cmljcy1hZG1pc3Npb24tZGVjbGFyYXRpb24tMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tk++4j3dncHUtaW5wdXQtcm9vdC1pZGVudGl0eS1yZXZpZXctMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+Tk++4j3dncHUtcmV0YWluZWQtaW5wdXQtc2lnbmF0dXJlLXByb3Bvc2FsLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9tZW1iZXItYWN0aW9uYnVzLXNob3J0LWNsb3NlLW5hdGl2ZS1yMi0yMDI2LTA4LTI4Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn6eq77iPbWVtYmVyLWFjdGlvbmJ1cy1zaG9ydC1jbG9zZS1zb3VyY2UtcjItMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+nqu+4j21lbWJlci1jb21tb24ta2VybmVsLWZ1bGwtcjItMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+nqu+4j21lbWJlci1wbHVnaW4tY2hlY2twb2ludC10YWlsLXI0LTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9tZW1iZXItcGx1Z2luLWNoZWNrcG9pbnQtdGFpbC1yNi0yMDI2LTA4LTI4Lm1kAC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn6eq77iPbWVtYmVyLXBsdWdpbi1jb25zdC1vd25lci1yZWQtcjEtMjAyNi0wOC0yOC5tZAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+nqu+4j21lbWJlci1wbHVnaW4tcmVzdGFydC1yZWdyZXNzaW9uLXI1LTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9tZW1iZXItcGx1Z2luLXJlc3RhcnQtdHdvLXIyLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9tZW1iZXItcGx1Z2luLXJlc3RhcnQtdHdvLXIzLTIwMjYtMDgtMjgubWQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9uYXRpdmUtYXJ0aWZhY3RzL3NlbWlvLW5leHRlc3QtNVFXd3hpL2JpbmFyaWVzLW1ldGFkYXRhLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9uYXRpdmUtYXJ0aWZhY3RzL3NlbWlvLW5leHRlc3QtUHo3UjdlL2JpbmFyaWVzLW1ldGFkYXRhLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9uYXRpdmUtYXJ0aWZhY3RzL3NlbWlvLW5leHRlc3QtYWVNTUNUL2JpbmFyaWVzLW1ldGFkYXRhLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9uYXRpdmUtYXJ0aWZhY3RzL3NlbWlvLW5leHRlc3QtbHU0TE52L2JpbmFyaWVzLW1ldGFkYXRhLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9uYXRpdmUtYXJ0aWZhY3RzL3NlbWlvLW5leHRlc3QtcE1nYUpXL2JpbmFyaWVzLW1ldGFkYXRhLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9uYXRpdmUtYXJ0aWZhY3RzL3NlbWlvLW5leHRlc3Qtd1VVUDd2L2JpbmFyaWVzLW1ldGFkYXRhLmpzb24ALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9yZW5kZXJlci1wYXJlbnQtc2xvdC1jb250cmFjdC1yMTEtMjAyNi0wOC0yOC50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9yZW5kZXJlci1wb29sLWNvbXBvc2l0aW9uLXIxMC0yMDI2LTA4LTI4LnR4dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+nqu+4j3JlbmRlcmVyLXBvb2wtY29uc3RydWN0aW9uLWdyZWVuLXI5LTIwMjYtMDgtMjgudHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn6eq77iPcmVuZGVyZXItcG9vbC1jb25zdHJ1Y3Rpb24tcmVkLXI4LTIwMjYtMDgtMjgudHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn6eq77iPcmVuZGVyZXItc2NvcGUtc2xvdC1yZWQtcjEyLTIwMjYtMDgtMjgudHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn6eq77iPcmVuZGVyZXItc2hhcmVkLWNlbGwtbWV0YWRhdGEtcjctMjAyNi0wOC0yOC50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9zaGFyZWQtcmVzaWRlbnQtYWxpYXMtcmVkLXIyNi0yMDI2LTA4LTI4LnR4dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+nqu+4j3NoYXJlZC1yZXNpZGVudC1jZWxsLWJvb3RzdHJhcC1yMjMtMjAyNi0wOC0yOC50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9zaGFyZWQtcmVzaWRlbnQtY2VsbC1yZXNvdXJjZS1ncmVlbi1yMjctMjAyNi0wOC0yOC50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9zaGFyZWQtcmVzaWRlbnQtY2VsbC1yZXNvdXJjZS1yMjUtMjAyNi0wOC0yOC50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9zaGFyZWQtcmVzaWRlbnQtY2VsbC1yZXNvdXJjZS1yZWQtcjI0LTIwMjYtMDgtMjgudHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn6eq77iPc2hhcmVkLXJlc2lkZW50LWRpc3BhdGNoLXIzMS0yMDI2LTA4LTI4LnR4dAAu8J+nrHNlbWlvL/CfppHvuI9yZXBvL/CfjqvvuI90aWNrZXRzL/CfjobvuI8yNi/wn4yZ77iPMDgv4piA77iPMjAvSU5URVJBQ1RJVkUtSk9CLVJVTlRJTUUtUkVGQUNUT1Iv8J+nqu+4j3NoYXJlZC1yZXNpZGVudC1kaXNwYXRjaC1yMzItMjAyNi0wOC0yOC50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9zaGFyZWQtcmVzaWRlbnQtZGlzcGF0Y2gtcjMzLTIwMjYtMDgtMjgudHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yMC9JTlRFUkFDVElWRS1KT0ItUlVOVElNRS1SRUZBQ1RPUi/wn6eq77iPc2hhcmVkLXJlc2lkZW50LWRpc3BhdGNoLXJlZC1yMzAtMjAyNi0wOC0yOC50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9zaGFyZWQtcmVzaWRlbnQtcXVhcmFudGluZS1ncmVlbi1yMjktMjAyNi0wOC0yOC50eHQALvCfp6xzZW1pby/wn6aR77iPcmVwby/wn46r77iPdGlja2V0cy/wn46G77iPMjYv8J+Mme+4jzA4L+KYgO+4jzIwL0lOVEVSQUNUSVZFLUpPQi1SVU5USU1FLVJFRkFDVE9SL/Cfp6rvuI9zaGFyZWQtcmVzaWRlbnQtcXVhcmFudGluZS1yZWQtcjI4LTIwMjYtMDgtMjgudHh0AC7wn6esc2VtaW8v8J+mke+4j3JlcG8v8J+Oq++4j3RpY2tldHMv8J+Ohu+4jzI2L/CfjJnvuI8wOC/imIDvuI8yNy9TVUJTRVQtU0NPUEVELUVYVEVSTkFMLU9SQUNMRS1NVVRBVElPTi1URVNUSU5HL/Cfk5PvuI9idWlsZC11bmJsb2NrLm1kAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv4piB77iPbGFzL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzEuMC/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/imIHvuI9wbHkv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMS4wL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/CfjJDvuI9odG1sL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzUv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Mpu+4j2Vwdy/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI9lbmVyZ3lwbHVzL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/CfjpLvuI96aXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMi4wL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/CfjpLvuI96aXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMi4wL/CfqobvuI9zdWJzZXRzL+Kcs++4j2lzbzIxMzIwL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Onu+4j2dpZi/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI84N2Ev8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Onu+4j2dpZi/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI84OWEv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Onu+4j3BwdHgv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPZWNtYS0zNzYv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Onu+4j3BwdHgv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPZWNtYS0zNzYv8J+qhu+4j3N1YnNldHMv4pyz77iPc3RyaWN0L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Onu+4j3BwdHgv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPZWNtYS0zNzYv8J+qhu+4j3N1YnNldHMv4pyz77iPdHJhbnNpdGlvbmFsL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Ope+4j21wNC/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI9pc29ibWZmL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/CfjqjvuI9zdmcv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMS4xL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/CfjqjvuI9zdmcv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMS4xL/CfqobvuI9zdWJzZXRzL+Kcs++4j2Jhc2ljL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+OqO+4j3N2Zy/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI8xLjEv8J+qhu+4j3N1YnNldHMv4pyz77iPdGlueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/CfjrXvuI9tcDMv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPbXBlZzEtbGF5ZXIzL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfj5fvuI9pZmMv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMngzL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfj5fvuI9pZmMv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMngzL/CfqobvuI9zdWJzZXRzL+Kcs++4j2NvYmllL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Pl++4j2lmYy/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI8yeDMv8J+qhu+4j3N1YnNldHMv4pyz77iPY3YyMC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfj5fvuI9pZmMv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMngzL/CfqobvuI9zdWJzZXRzL+Kcs++4j3Nhdi/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfj5fvuI9pZmMv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPNC/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5Ks77iPYmNmL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzIuMS/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5K+77iPYmluYXJ5L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3Jhdy/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5OE77iPcGRmL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzEuNC/wn6qG77iPc3Vic2V0cy/inLPvuI9hL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+ThO+4j3BkZi/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI8xLjQv8J+qhu+4j3N1YnNldHMv4pyz77iPYmFzZS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk4TvuI9wZGYv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMS40L/CfqobvuI9zdWJzZXRzL+Kcs++4j3gv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5OE77iPcGRmL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzEuNy/wn6qG77iPc3Vic2V0cy/inLPvuI9hL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+ThO+4j3BkZi/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI8xLjcv8J+qhu+4j3N1YnNldHMv4pyz77iPYmFzZS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk4TvuI9wZGYv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMS43L/CfqobvuI9zdWJzZXRzL+Kcs++4j2Uv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5OE77iPcGRmL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzEuNy/wn6qG77iPc3Vic2V0cy/inLPvuI9oL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+ThO+4j3BkZi/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI8xLjcv8J+qhu+4j3N1YnNldHMv4pyz77iPdWEv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5OE77iPcGRmL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzEuNy/wn6qG77iPc3Vic2V0cy/inLPvuI92dC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk4TvuI9wZGYv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMS43L/CfqobvuI9zdWJzZXRzL+Kcs++4j3gv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5OK77iPY3N2L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3JmYzQxODAv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+TkO+4j3N0ZXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPYXAyMTQv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+TkO+4j3N0ZXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPYXAyMTQv8J+qhu+4j3N1YnNldHMv4pyz77iPY2MxL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+TkO+4j3N0ZXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPYXAyMTQv8J+qhu+4j3N1YnNldHMv4pyz77iPY2MyL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+TkO+4j3N0ZXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPYXAyMTQv8J+qhu+4j3N1YnNldHMv4pyz77iPY2MzL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+TkO+4j3N0ZXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPYXAyMTQv8J+qhu+4j3N1YnNldHMv4pyz77iPY2M0L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+TkO+4j3N0ZXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPYXAyMTQv8J+qhu+4j3N1YnNldHMv4pyz77iPY2M1L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+TkO+4j3N0ZXAv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPYXAyMTQv8J+qhu+4j3N1YnNldHMv4pyz77iPY2M2L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Tke+4j3Rzdi/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI9pYW5hL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk5XvuI94bHN4L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j2VjbWEtMzc2L/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk5XvuI94bHN4L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j2VjbWEtMzc2L/CfqobvuI9zdWJzZXRzL+Kcs++4j3N0cmljdC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk5XvuI94bHN4L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j2VjbWEtMzc2L/CfqobvuI9zdWJzZXRzL+Kcs++4j3RyYW5zaXRpb25hbC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk5zvuI9kb2N4L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j2VjbWEtMzc2L/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk5zvuI9kb2N4L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j2VjbWEtMzc2L/CfqobvuI9zdWJzZXRzL+Kcs++4j3N0cmljdC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk5zvuI9kb2N4L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j2VjbWEtMzc2L/CfqobvuI9zdWJzZXRzL+Kcs++4j3RyYW5zaXRpb25hbC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk53vuI9tZC/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI9jb21tb25tYXJrL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk7B4bWwv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMS4wL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfk7B4bWwv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMS4wL/CfqobvuI9zdWJzZXRzL+Kcs++4j3ZhbGlkL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Tt++4j2pwZy/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI9qZmlmLTEuMDEv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Tt++4j2pwZy/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI9qZmlmLTEuMDEv8J+qhu+4j3N1YnNldHMv4pyz77iPYmFzZWxpbmUv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5O377iPcG5nL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzEuMi/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5O877iPYXZpL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzEuMC/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5SK77iPd2F2L/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3JpZmYtcGNtL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/CflKPvuI9qc29uL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3JmYzgyNTkv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+Uo++4j2pzb24v8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPcmZjODI1OS/wn6qG77iPc3Vic2V0cy/inLPvuI9pLWpzb24v8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5aK77iPZHdnL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j2FjMTAxOC/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5aK77iPZHdnL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j2FjMTAyNC/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5aK77iPZHhmL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3IxMi/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5a877iPYm1wL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3YzL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/CflrzvuI90aWZmL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4jzYuMC/wn6qG77iPc3Vic2V0cy/inLPvuI9hbnkv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5a877iPdGlmZi/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI82LjAv8J+qhu+4j3N1YnNldHMv4pyz77iPYmFzZWxpbmUv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn5ec77iPZGVmbGF0ZS/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI9yZmMxOTUwL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfn6rvuI9zdGwv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPYXNjaWkv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+niu+4j2dsdGYv8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMi4wL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfp4rvuI9vYmov8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPMy4wL/CfqobvuI9zdWJzZXRzL+Kcs++4j2FueS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfp7/vuI9zZW1pby/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI92MS/wn6qG77iPc3Vic2V0cy/inLPvuI9hbmltYXRpb24v8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn6e/77iPc2VtaW8v8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPdjEv8J+qhu+4j3N1YnNldHMv4pyz77iPYW55L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+nv++4j3NlbWlvL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3YxL/CfqobvuI9zdWJzZXRzL+Kcs++4j2F1ZGlvL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+nv++4j3NlbWlvL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3YxL/CfqobvuI9zdWJzZXRzL+Kcs++4j2JyZXAv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn6e/77iPc2VtaW8v8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPdjEv8J+qhu+4j3N1YnNldHMv4pyz77iPY2FkL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+nv++4j3NlbWlvL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3YxL/CfqobvuI9zdWJzZXRzL+Kcs++4j2RvY3VtZW50L/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+nv++4j3NlbWlvL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3YxL/CfqobvuI9zdWJzZXRzL+Kcs++4j2RyYXdpbmcv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn6e/77iPc2VtaW8v8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPdjEv8J+qhu+4j3N1YnNldHMv4pyz77iPZmxvdy/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfp7/vuI9zZW1pby/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI92MS/wn6qG77iPc3Vic2V0cy/inLPvuI9ncmFwaC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfp7/vuI9zZW1pby/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI92MS/wn6qG77iPc3Vic2V0cy/inLPvuI9pbWFnZS/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfp7/vuI9zZW1pby/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI92MS/wn6qG77iPc3Vic2V0cy/inLPvuI9raXQv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn6e/77iPc2VtaW8v8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPdjEv8J+qhu+4j3N1YnNldHMv4pyz77iPbWVzaC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfp7/vuI9zZW1pby/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI92MS/wn6qG77iPc3Vic2V0cy/inLPvuI9tb2RlbC/wn6es77iPc2NoZW1hL/Cfp6zvuI9tdXRhdGlvbnMv8J+mgO+4jy5ycwDinI/vuI9zL/CflIzvuI9wbHVnaW5zL/Cfl4TvuI9zdGRpby/wn5e/77iPYXJ0aWZhY3RzL/Cfp7/vuI9zZW1pby/wn4+F77iPc3RhbmRhcmRzL/CflJbvuI92MS/wn6qG77iPc3Vic2V0cy/inLPvuI9vYmplY3Qv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn6e/77iPc2VtaW8v8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPdjEv8J+qhu+4j3N1YnNldHMv4pyz77iPcHJlc2VudGF0aW9uL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+nv++4j3NlbWlvL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3YxL/CfqobvuI9zdWJzZXRzL+Kcs++4j3RhYmxlL/Cfp6zvuI9zY2hlbWEv8J+nrO+4j211dGF0aW9ucy/wn6aA77iPLnJzAOKcj++4j3Mv8J+UjO+4j3BsdWdpbnMv8J+XhO+4j3N0ZGlvL/Cfl7/vuI9hcnRpZmFjdHMv8J+nv++4j3NlbWlvL/Cfj4XvuI9zdGFuZGFyZHMv8J+Ulu+4j3YxL/CfqobvuI9zdWJzZXRzL+Kcs++4j3RleHQv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn6e/77iPc2VtaW8v8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPdjEv8J+qhu+4j3N1YnNldHMv4pyz77iPdmFsdWUv8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA4pyP77iPcy/wn5SM77iPcGx1Z2lucy/wn5eE77iPc3RkaW8v8J+Xv++4j2FydGlmYWN0cy/wn6e/77iPc2VtaW8v8J+Phe+4j3N0YW5kYXJkcy/wn5SW77iPdjEv8J+qhu+4j3N1YnNldHMv4pyz77iPdmlkZW8v8J+nrO+4j3NjaGVtYS/wn6es77iPbXV0YXRpb25zL/CfpoDvuI8ucnMA8J+nsO+4j2ZyYW1ld29yay/wn5So77iPbW9kdWxlcy/wn5ax77iPdWkv8J+Wpe+4j2hvc3Qv8J+Tpe+4j2lucHV0L/Cfjp/vuI9hZG1pc3Npb24v8J+TnO+4j3NjcmlwdC50cwDwn6ew77iPZnJhbWV3b3JrL/CflKjvuI9tb2R1bGVzL/CflrHvuI91aS/wn5al77iPaG9zdC/wn5Ol77iPaW5wdXQv8J+On++4j2FkbWlzc2lvbi/wn6eq77iPY29tcG9uZW50LnJzAPCfp7DvuI9mcmFtZXdvcmsv8J+UqO+4j21vZHVsZXMv8J+Wse+4j3VpL/CflqXvuI9ob3N0L/Cfk6XvuI9pbnB1dC/wn46f77iPYWRtaXNzaW9uL/Cfp6rvuI90ZXN0cy/wn5Sj77iPLmpzb24A8J+nsO+4j2ZyYW1ld29yay/wn5So77iPbW9kdWxlcy/wn5ax77iPdWkv8J+Wpe+4j2hvc3Qv8J+Tpe+4j2lucHV0L/Cfjp/vuI9hZG1pc3Npb24v8J+nrO+4j3NjaGVtYS/wn5Sj77iPLmpzb24A8J+nsO+4j2ZyYW1ld29yay/wn5So77iPbW9kdWxlcy/wn5ax77iPdWkv8J+Wpe+4j2hvc3Qv8J+Tpu+4j3BhY2thZ2VzL/CfpoDvuI9ydXN0L/Cfk4vvuI9wcm9qZWN0Lmpzb24A8J+nsO+4j2ZyYW1ld29yay/wn5So77iPbW9kdWxlcy/wn5ax77iPdWkv8J+Wpe+4j2hvc3Qv8J+Tpu+4j3BhY2thZ2VzL/CfpoDvuI9ydXN0L/Cfk5zvuI9zY3JpcHQudHMA8J+nsO+4j2ZyYW1ld29yay/wn5So77iPbW9kdWxlcy/wn5ax77iPdWkv8J+nrO+4j2NvbnRyYWN0L/Cfp7XvuI9yZXRhaW5lZC/wn5K+77iPcmVzaWRlbnQv8J+TqO+4j3Nsb3Qv8J+nqu+4j2ZpeHR1cmUuanNvbgDwn6ew77iPZnJhbWV3b3JrL/CflKjvuI9tb2R1bGVzL/CflrHvuI91aS/wn6es77iPY29udHJhY3Qv8J+nte+4j3JldGFpbmVkL/Cfkr7vuI9yZXNpZGVudC/wn5Oo77iPc2xvdC/wn6eq77iPc2NoZW1hLmpzb24A8J+nsO+4j2ZyYW1ld29yay/wn5So77iPbW9kdWxlcy/wn5ax77iPdWkv8J+nrO+4j2NvbnRyYWN0L/Cfp7XvuI9yZXRhaW5lZC/wn5K+77iPcmVzaWRlbnQv8J+TqO+4j3Nsb3Qv8J+nrO+4j2NvbnRyYWN0Lmpzb24A8J+nsO+4j2ZyYW1ld29yay/wn5So77iPbW9kdWxlcy/wn5ax77iPdWkv8J+nrO+4j2NvbnRyYWN0L/Cfp7XvuI9yZXRhaW5lZC/wn5K+77iPcmVzaWRlbnQv8J+TqO+4j3Nsb3Qv8J+nrO+4j3NjaGVtYS5qc29uAPCfp7DvuI9mcmFtZXdvcmsv8J+bje+4j3Byb2R1Y3RzL/CfppHvuI9yZXBvL/CflKjvuI9tb2R1bGVzL/Cfk5rvuI9saWJyYXJ5L/Cfp6rvuI90ZXN0cy/wn6eq77iPbWFya2Rvd24taW5saW5lLXJlZmVyZW5jZXMv8J+fpu+4jy50cwDwn6ew77iPZnJhbWV3b3JrL/Cfm43vuI9wcm9kdWN0cy/wn6aR77iPcmVwby/wn5So77iPbW9kdWxlcy/wn5Oa77iPbGlicmFyeS/wn6eq77iPdGVzdHMv8J+nqu+4j3BhY2thZ2UtbGFuZ3VhZ2Uta2luZC1oYW5kb2ZmL/Cfp6rvuI91aS1ob3N0LXBhY2thZ2Uv8J+Uo++4jy5qc29uAPCfp7DvuI9mcmFtZXdvcmsv8J+bje+4j3Byb2R1Y3RzL/CfppHvuI9yZXBvL/CflKjvuI9tb2R1bGVzL/Cfk5rvuI9saWJyYXJ5L/Cfp6rvuI90ZXN0cy/wn6eq77iPcGFja2FnZS1sYW5ndWFnZS1raW5kLWhhbmRvZmYv8J+nqu+4j3VpLWhvc3QtcGFja2FnZS/wn6es77iPc2NoZW1hL/CflKPvuI8uanNvbgDwn6ew77iPZnJhbWV3b3JrL/Cfm43vuI9wcm9kdWN0cy/wn6aR77iPcmVwby/wn5So77iPbW9kdWxlcy/wn5Oa77iPbGlicmFyeS/wn6e577iPbm9ybWFsaXphdGlvbi/wn6eq77iPdGVzdHMv8J+nqu+4j3NvdXJjZS1hZG1pc3Npb24v8J+nqu+4j2lvL/CflKPvuI8uanNvbgDwn6ew77iPZnJhbWV3b3JrL/Cfm43vuI9wcm9kdWN0cy/wn6aR77iPcmVwby/wn5So77iPbW9kdWxlcy/wn5Oa77iPbGlicmFyeS/wn6e577iPbm9ybWFsaXphdGlvbi/wn6eq77iPdGVzdHMv8J+nqu+4j3NvdXJjZS1hZG1pc3Npb24v8J+nqu+4j2lvL/Cfn6bvuI8udHMA8J+nsO+4j2ZyYW1ld29yay/wn5uN77iPcHJvZHVjdHMv8J+mke+4j3JlcG8v8J+UqO+4j21vZHVsZXMv8J+Tmu+4j2xpYnJhcnkv8J+nue+4j25vcm1hbGl6YXRpb24v8J+nqu+4j3Rlc3RzL/Cfp6rvuI9zb3VyY2UtYWRtaXNzaW9uL/Cfp6rvuI9pby/wn6es77iPc2NoZW1hL/CflKPvuI8uanNvbgA="
  },
  "counts": {
    "records": 320,
    "invalid": 3,
    "terminalSlashOnly": 3,
    "otherUnsafe": 0
  },
  "invalidRecords": [
    {
      "index": 20,
      "byteOffset": 3168,
      "byteLength": 169,
      "escaped": "\".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-physical-integration-53/🧫️run-CtgJYd/🧪️fixture/\"",
      "hex": "2ef09fa7ac73656d696f2ff09fa691efb88f7265706f2ff09f8eabefb88f7469636b6574732ff09f8e86efb88f32362ff09f8c99efb88f30382fe29880efb88f31322f53454d414e5449432d4d55544154494f4e532d4f5645524841554c2ff09f9393efb88f61646d697373696f6e2d706879736963616c2d696e746567726174696f6e2d35332ff09fa7abefb88f72756e2d4374674a59642ff09fa7aaefb88f666978747572652f",
      "actualSafe": false,
      "reasonPredicateSafe": false,
      "reasons": [
        "emptySegments"
      ],
      "facts": {
        "empty": false,
        "leadingSlash": false,
        "drivePrefix": false,
        "backslash": false,
        "controls": false,
        "lossyUtf8Roundtrip": false,
        "emptySegments": [
          10
        ],
        "dotSegments": []
      },
      "terminalSlashOnly": true,
      "rawBytesMatchDecodedText": true
    },
    {
      "index": 23,
      "byteOffset": 3677,
      "byteLength": 169,
      "escaped": "\".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-physical-integration-53/🧫️run-Sjc4gE/🧪️fixture/\"",
      "hex": "2ef09fa7ac73656d696f2ff09fa691efb88f7265706f2ff09f8eabefb88f7469636b6574732ff09f8e86efb88f32362ff09f8c99efb88f30382fe29880efb88f31322f53454d414e5449432d4d55544154494f4e532d4f5645524841554c2ff09f9393efb88f61646d697373696f6e2d706879736963616c2d696e746567726174696f6e2d35332ff09fa7abefb88f72756e2d536a633467452ff09fa7aaefb88f666978747572652f",
      "actualSafe": false,
      "reasonPredicateSafe": false,
      "reasons": [
        "emptySegments"
      ],
      "facts": {
        "empty": false,
        "leadingSlash": false,
        "drivePrefix": false,
        "backslash": false,
        "controls": false,
        "lossyUtf8Roundtrip": false,
        "emptySegments": [
          10
        ],
        "dotSegments": []
      },
      "terminalSlashOnly": true,
      "rawBytesMatchDecodedText": true
    },
    {
      "index": 26,
      "byteOffset": 4186,
      "byteLength": 169,
      "escaped": "\".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️admission-physical-integration-53/🧫️run-o21R2r/🧪️fixture/\"",
      "hex": "2ef09fa7ac73656d696f2ff09fa691efb88f7265706f2ff09f8eabefb88f7469636b6574732ff09f8e86efb88f32362ff09f8c99efb88f30382fe29880efb88f31322f53454d414e5449432d4d55544154494f4e532d4f5645524841554c2ff09f9393efb88f61646d697373696f6e2d706879736963616c2d696e746567726174696f6e2d35332ff09fa7abefb88f72756e2d6f32315232722ff09fa7aaefb88f666978747572652f",
      "actualSafe": false,
      "reasonPredicateSafe": false,
      "reasons": [
        "emptySegments"
      ],
      "facts": {
        "empty": false,
        "leadingSlash": false,
        "drivePrefix": false,
        "backslash": false,
        "controls": false,
        "lossyUtf8Roundtrip": false,
        "emptySegments": [
          10
        ],
        "dotSegments": []
      },
      "terminalSlashOnly": true,
      "rawBytesMatchDecodedText": true
    }
  ],
  "classifications": [
    {
      "index": 0,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 1,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 2,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 3,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 4,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 5,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 6,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 7,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 8,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 9,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 10,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 11,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 12,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 13,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 14,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 15,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 16,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 17,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 18,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 19,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 20,
      "actualSafe": false,
      "reasonPredicateSafe": false,
      "terminalSlashOnly": true
    },
    {
      "index": 21,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 22,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 23,
      "actualSafe": false,
      "reasonPredicateSafe": false,
      "terminalSlashOnly": true
    },
    {
      "index": 24,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 25,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 26,
      "actualSafe": false,
      "reasonPredicateSafe": false,
      "terminalSlashOnly": true
    },
    {
      "index": 27,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 28,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 29,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 30,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 31,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 32,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 33,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 34,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 35,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 36,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 37,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 38,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 39,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 40,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 41,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 42,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 43,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 44,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 45,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 46,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 47,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 48,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 49,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 50,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 51,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 52,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 53,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 54,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 55,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 56,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 57,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 58,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 59,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 60,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 61,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 62,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 63,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 64,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 65,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 66,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 67,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 68,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 69,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 70,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 71,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 72,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 73,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 74,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 75,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 76,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 77,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 78,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 79,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 80,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 81,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 82,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 83,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 84,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 85,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 86,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 87,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 88,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 89,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 90,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 91,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 92,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 93,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 94,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 95,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 96,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 97,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 98,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 99,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 100,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 101,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 102,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 103,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 104,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 105,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 106,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 107,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 108,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 109,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 110,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 111,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 112,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 113,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 114,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 115,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 116,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 117,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 118,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 119,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 120,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 121,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 122,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 123,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 124,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 125,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 126,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 127,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 128,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 129,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 130,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 131,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 132,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 133,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 134,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 135,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 136,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 137,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 138,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 139,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 140,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 141,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 142,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 143,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 144,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 145,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 146,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 147,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 148,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 149,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 150,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 151,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 152,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 153,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 154,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 155,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 156,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 157,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 158,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 159,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 160,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 161,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 162,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 163,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 164,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 165,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 166,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 167,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 168,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 169,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 170,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 171,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 172,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 173,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 174,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 175,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 176,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 177,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 178,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 179,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 180,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 181,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 182,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 183,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 184,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 185,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 186,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 187,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 188,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 189,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 190,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 191,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 192,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 193,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 194,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 195,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 196,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 197,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 198,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 199,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 200,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 201,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 202,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 203,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 204,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 205,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 206,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 207,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 208,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 209,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 210,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 211,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 212,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 213,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 214,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 215,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 216,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 217,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 218,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 219,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 220,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 221,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 222,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 223,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 224,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 225,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 226,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 227,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 228,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 229,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 230,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 231,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 232,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 233,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 234,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 235,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 236,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 237,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 238,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 239,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 240,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 241,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 242,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 243,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 244,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 245,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 246,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 247,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 248,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 249,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 250,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 251,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 252,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 253,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 254,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 255,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 256,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 257,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 258,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 259,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 260,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 261,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 262,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 263,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 264,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 265,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 266,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 267,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 268,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 269,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 270,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 271,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 272,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 273,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 274,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 275,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 276,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 277,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 278,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 279,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 280,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 281,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 282,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 283,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 284,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 285,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 286,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 287,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 288,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 289,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 290,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 291,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 292,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 293,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 294,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 295,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 296,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 297,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 298,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 299,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 300,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 301,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 302,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 303,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 304,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 305,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 306,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 307,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 308,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 309,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 310,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 311,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 312,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 313,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 314,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 315,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 316,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 317,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 318,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    },
    {
      "index": 319,
      "actualSafe": true,
      "reasonPredicateSafe": true,
      "terminalSlashOnly": false
    }
  ],
  "sourceStable": false,
  "checks": [
    {
      "name": "instrumented original module identity preserved",
      "ok": true,
      "detail": null
    },
    {
      "name": "after-instrumented-import: exact source endpoints",
      "ok": true,
      "detail": null
    },
    {
      "name": "actual original loader consumed exact current taxonomy",
      "ok": true,
      "detail": null
    },
    {
      "name": "before-one-git-call: exact source endpoints",
      "ok": true,
      "detail": null
    },
    {
      "name": "all in-memory reason predicates agree with actual extracted safe-path helper",
      "ok": true,
      "detail": null
    },
    {
      "name": "exactly one actual Git call",
      "ok": true,
      "detail": null
    },
    {
      "name": "complete Git bytes returned within bounded call",
      "ok": true,
      "detail": null
    },
    {
      "name": "final: exact source endpoints",
      "ok": false,
      "detail": null
    }
  ],
  "failure": null,
  "captureComplete": false,
  "interpretationLimit": "Terminal-slash-only means Git supplied a record whose sole current safe-path failure is the empty final segment. This is consistent with Git terminal directory/repository reporting; repository contents were not probed. Raw bytes and command flags remain the authority, not a silently trimmed path.",
  "normalStderrCapture": "execFileSync returns stdout; stderr is preserved on command failure when supplied by its error object."
}
```
