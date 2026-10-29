# Live Process Inspection

Read-only /proc samples two seconds apart. CPU ticks are Linux scheduler accounting units. Empty per-thread children means no child processes observed. Thread names and wait channels are kernel facts, not inference progress counters.

```json
{
  "at": "2026-10-28T18:45:20.399Z",
  "before": [
    {
      "id": 211237,
      "parent": 211110,
      "state": "R",
      "ticks": 304169,
      "wchan": "0",
      "io": "rchar: 19585810872\nwchar: 55719193\nsyscr: 2563987\nsyscw: 960526\nread_bytes: 8523010048\nwrite_bytes: 81903616\ncancelled_write_bytes: 9715712\n",
      "argvTail": [
        "--eval",
        "[loader omitted]",
        "--",
        "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5",
        "run",
        "@semio-tech/framework-os-mcp-rs:build",
        "--output=stream",
        "--parallel=1",
        ""
      ],
      "children": ""
    },
    {
      "id": 211110,
      "parent": 171027,
      "state": "S",
      "ticks": 4196,
      "wchan": "do_epoll_wait",
      "io": "rchar: 280890667\nwchar: 21963737\nsyscr: 195812\nsyscw: 1680935\nread_bytes: 994295808\nwrite_bytes: 11079680\ncancelled_write_bytes: 217088\n",
      "argvTail": [
        "--eval",
        "[loader omitted]",
        "--",
        "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5",
        "run",
        "@semio-tech/framework-os-mcp-rs:build",
        "--output=stream",
        "--parallel=1",
        ""
      ],
      "children": "211237"
    },
    {
      "id": 229702,
      "parent": 229429,
      "state": "S",
      "ticks": 73066,
      "wchan": "do_epoll_wait",
      "io": "rchar: 21462857716\nwchar: 136742618\nsyscr: 1536625\nsyscw: 4076744\nread_bytes: 8639400960\nwrite_bytes: 111378432\ncancelled_write_bytes: 13291520\n",
      "argvTail": [
        "--eval",
        "[loader omitted]",
        "--",
        "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd",
        "--tool=semio",
        "run",
        "@semio-tech/print:build-thesis",
        "--parallel=1",
        ""
      ],
      "children": ""
    },
    {
      "id": 229429,
      "parent": 214878,
      "state": "S",
      "ticks": 7798,
      "wchan": "do_epoll_wait",
      "io": "rchar: 234090029\nwchar: 49354127\nsyscr: 142109\nsyscw: 5192603\nread_bytes: 828880896\nwrite_bytes: 9928704\ncancelled_write_bytes: 352256\n",
      "argvTail": [
        "--eval",
        "[loader omitted]",
        "--",
        "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd",
        "--tool=semio",
        "run",
        "@semio-tech/print:build-thesis",
        "--parallel=1",
        ""
      ],
      "children": "229702"
    }
  ],
  "after": [
    {
      "id": 211237,
      "parent": 211110,
      "state": "R",
      "ticks": 304369,
      "wchan": "0",
      "io": "rchar: 19585882505\nwchar: 55719692\nsyscr: 2565348\nsyscw: 960529\nread_bytes: 8523153408\nwrite_bytes: 81903616\ncancelled_write_bytes: 9715712\n",
      "argvTail": [
        "--eval",
        "[loader omitted]",
        "--",
        "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5",
        "run",
        "@semio-tech/framework-os-mcp-rs:build",
        "--output=stream",
        "--parallel=1",
        ""
      ],
      "children": ""
    },
    {
      "id": 211110,
      "parent": 171027,
      "state": "S",
      "ticks": 4196,
      "wchan": "do_epoll_wait",
      "io": "rchar: 280890667\nwchar: 21964236\nsyscr: 195812\nsyscw: 1680937\nread_bytes: 994299904\nwrite_bytes: 11079680\ncancelled_write_bytes: 217088\n",
      "argvTail": [
        "--eval",
        "[loader omitted]",
        "--",
        "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5",
        "run",
        "@semio-tech/framework-os-mcp-rs:build",
        "--output=stream",
        "--parallel=1",
        ""
      ],
      "children": "211237"
    },
    {
      "id": 229702,
      "parent": 229429,
      "state": "S",
      "ticks": 73068,
      "wchan": "do_epoll_wait",
      "io": "rchar: 21463046446\nwchar: 136743032\nsyscr: 1536876\nsyscw: 4076746\nread_bytes: 8639507456\nwrite_bytes: 111378432\ncancelled_write_bytes: 13291520\n",
      "argvTail": [
        "--eval",
        "[loader omitted]",
        "--",
        "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd",
        "--tool=semio",
        "run",
        "@semio-tech/print:build-thesis",
        "--parallel=1",
        ""
      ],
      "children": ""
    },
    {
      "id": 229429,
      "parent": 214878,
      "state": "S",
      "ticks": 7798,
      "wchan": "do_epoll_wait",
      "io": "rchar: 234090029\nwchar: 49354541\nsyscr: 142109\nsyscw: 5192605\nread_bytes: 828880896\nwrite_bytes: 9928704\ncancelled_write_bytes: 352256\n",
      "argvTail": [
        "--eval",
        "[loader omitted]",
        "--",
        "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd",
        "--tool=semio",
        "run",
        "@semio-tech/print:build-thesis",
        "--parallel=1",
        ""
      ],
      "children": "229702"
    }
  ],
  "threads": [
    {
      "pid": 211237,
      "tid": "211237",
      "name": "bun",
      "wait": "0",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211238",
      "name": "Bun Pool 0",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211239",
      "name": "mi-scavenger",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211240",
      "name": "Bun Pool 1",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211290",
      "name": "Bun Pool 2",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211292",
      "name": "Bun Pool 3",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211293",
      "name": "Bun Pool 4",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211294",
      "name": "Bun Pool 5",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211295",
      "name": "Bun Pool 6",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211296",
      "name": "Bun Pool 7",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211297",
      "name": "Bun Pool 8",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211299",
      "name": "Bun Pool 9",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211300",
      "name": "Bun Pool 10",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "211345",
      "name": "Bun Pool 11",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 211237,
      "tid": "254070",
      "name": "JSCWarmUp",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229702",
      "name": "bun",
      "wait": "do_epoll_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229703",
      "name": "Bun Pool 0",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229704",
      "name": "mi-scavenger",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229705",
      "name": "Bun Pool 1",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229709",
      "name": "Bun Pool 2",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229710",
      "name": "Bun Pool 3",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229711",
      "name": "Bun Pool 4",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229712",
      "name": "Bun Pool 5",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229713",
      "name": "Bun Pool 6",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229714",
      "name": "Bun Pool 7",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229715",
      "name": "Bun Pool 8",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229716",
      "name": "Bun Pool 9",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229717",
      "name": "Bun Pool 10",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "229779",
      "name": "Bun Pool 11",
      "wait": "futex_do_wait",
      "children": ""
    },
    {
      "pid": 229702,
      "tid": "254059",
      "name": "JSCWarmUp",
      "wait": "futex_do_wait",
      "children": ""
    }
  ]
}
```

## Live Lease Observation

```json
{
  "version": 1,
  "directory": "/home/deck/semio/.cache/b/l/m/data",
  "atMs": 1793213141735,
  "holders": [
    {
      "resource": "native-fixed-image:/home/deck/semio/.cache/b/o",
      "hash": "4a0990e349d6eca23b442cbbe2967bb1883e4cf064dfe9c187e755b66a1b905b",
      "mode": "shared",
      "pid": 211237,
      "task": 7,
      "sinceMs": 1793207143381,
      "heldSeconds": 5998,
      "class": "image",
      "rank": 7,
      "source": "registry",
      "command": "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5 run @semio-tech/framework-os-mcp-rs:build --output=stream --parallel=1",
      "session": "12974ae06f6b.a2kddq7xlne",
      "agent": "codex-fad27566"
    },
    {
      "resource": "native-fixed-image:/home/deck/semio/.cache/b/d",
      "hash": "3da0a8df5d3daa5753061c9c3786e24c8c2bb9e86451a970e57bfe2fb343ef5b",
      "mode": "shared",
      "pid": 211237,
      "task": 27,
      "sinceMs": 1793207143538,
      "heldSeconds": 5998,
      "class": "image",
      "rank": 7,
      "source": "registry",
      "command": "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5 run @semio-tech/framework-os-mcp-rs:build --output=stream --parallel=1",
      "session": "12974ae06f6b.a2kddq7xlne",
      "agent": "codex-fad27566"
    },
    {
      "resource": "native-fixed-image:/home/deck/semio/.cache/b/g",
      "hash": "70033059822fe0125c777ae951afb7956823995d6dd89c80fafc12ecfc1a7d46",
      "mode": "shared",
      "pid": 211237,
      "task": 116,
      "sinceMs": 1793208276069,
      "heldSeconds": 4865,
      "class": "image",
      "rank": 7,
      "source": "registry",
      "command": "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5 run @semio-tech/framework-os-mcp-rs:build --output=stream --parallel=1",
      "session": "12974ae06f6b.a2kddq7xlne",
      "agent": "codex-fad27566"
    },
    {
      "resource": "repository-graph-facts:/home/deck/semio:shard:7",
      "hash": "35daf78406392ae7ff2c673f4b0b221762e32b586ba47c2fae55b0cf74b543ff",
      "mode": "shared",
      "pid": 211237,
      "task": 120,
      "sinceMs": 1793208276453,
      "heldSeconds": 4865,
      "class": "graph",
      "rank": 4,
      "source": "registry",
      "command": "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5 run @semio-tech/framework-os-mcp-rs:build --output=stream --parallel=1",
      "session": "12974ae06f6b.a2kddq7xlne",
      "agent": "codex-fad27566"
    },
    {
      "resource": "native-fixed-image:/home/deck/semio/.cache/b/o",
      "hash": "4a0990e349d6eca23b442cbbe2967bb1883e4cf064dfe9c187e755b66a1b905b",
      "mode": "shared",
      "pid": 229702,
      "task": 7,
      "sinceMs": 1793209168745,
      "heldSeconds": 3972,
      "class": "image",
      "rank": 7,
      "source": "registry",
      "command": "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd --tool=semio run @semio-tech/print:build-thesis --parallel=1",
      "session": "12974ae06f6b.a1lx59b1pwg",
      "agent": "01a18350-b1ec-7522-9d60-061b9baee7a0"
    },
    {
      "resource": "native-fixed-image:/home/deck/semio/.cache/b/d",
      "hash": "3da0a8df5d3daa5753061c9c3786e24c8c2bb9e86451a970e57bfe2fb343ef5b",
      "mode": "shared",
      "pid": 229702,
      "task": 27,
      "sinceMs": 1793209168914,
      "heldSeconds": 3972,
      "class": "image",
      "rank": 7,
      "source": "registry",
      "command": "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd --tool=semio run @semio-tech/print:build-thesis --parallel=1",
      "session": "12974ae06f6b.a1lx59b1pwg",
      "agent": "01a18350-b1ec-7522-9d60-061b9baee7a0"
    },
    {
      "resource": "native-fixed-image:/home/deck/semio/.cache/b/g",
      "hash": "70033059822fe0125c777ae951afb7956823995d6dd89c80fafc12ecfc1a7d46",
      "mode": "shared",
      "pid": 229702,
      "task": 139,
      "sinceMs": 1793209676117,
      "heldSeconds": 3465,
      "class": "image",
      "rank": 7,
      "source": "registry",
      "command": "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd --tool=semio run @semio-tech/print:build-thesis --parallel=1",
      "session": "12974ae06f6b.a1lx59b1pwg",
      "agent": "01a18350-b1ec-7522-9d60-061b9baee7a0"
    },
    {
      "resource": "repository-graph-facts:/home/deck/semio:shard:7",
      "hash": "35daf78406392ae7ff2c673f4b0b221762e32b586ba47c2fae55b0cf74b543ff",
      "mode": "shared",
      "pid": 229702,
      "task": 143,
      "sinceMs": 1793209676417,
      "heldSeconds": 3465,
      "class": "graph",
      "rank": 4,
      "source": "registry",
      "command": "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd --tool=semio run @semio-tech/print:build-thesis --parallel=1",
      "session": "12974ae06f6b.a1lx59b1pwg",
      "agent": "01a18350-b1ec-7522-9d60-061b9baee7a0"
    },
    {
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0",
      "hash": "a949bf0423e69f6789b69aba4613cf22cbdc990d3a8ef69fdaf194d91660fd3c",
      "mode": "exclusive",
      "pid": 211237,
      "task": 120,
      "sinceMs": 1793211214587,
      "heldSeconds": 1927,
      "class": "unclassified",
      "rank": 0,
      "source": "registry",
      "command": "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5 run @semio-tech/framework-os-mcp-rs:build --output=stream --parallel=1",
      "session": "12974ae06f6b.a2kddq7xlne",
      "agent": "codex-fad27566"
    },
    {
      "resource": "graph-claim:/home/deck/semio/.cache/b/g/m/generations:49620f499d3a2883fbe02d40",
      "hash": "de6034be993cc704495881704f83802a6f6e9259b281c91171380d628fe448c1",
      "mode": "exclusive",
      "pid": 211237,
      "task": 120,
      "sinceMs": 1793211214596,
      "heldSeconds": 1927,
      "class": "unclassified",
      "rank": 0,
      "source": "registry",
      "command": "09779154bf5ca995db25160c46cbe069c998a0c103f256b5a394ba45dd4eb7f5 run @semio-tech/framework-os-mcp-rs:build --output=stream --parallel=1",
      "session": "12974ae06f6b.a2kddq7xlne",
      "agent": "codex-fad27566"
    }
  ],
  "waiters": [
    {
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0",
      "hash": "a949bf0423e69f6789b69aba4613cf22cbdc990d3a8ef69fdaf194d91660fd3c",
      "mode": "shared",
      "pid": 249683,
      "task": 123,
      "sinceMs": 1793213137612,
      "waitedSeconds": 4,
      "class": "unclassified",
      "rank": 0,
      "source": "registry",
      "position": 4,
      "queued": 6,
      "blockers": [
        "211237/120"
      ],
      "command": "f1f17c65ed4d13df7382a865f1624c25b1c583783b2fa9cd2d77f9ec14721e85 run @teaching/architecture-course:test-e2e:release --parallel=1 -- --serial",
      "session": "12974ae06f6b.a2cb10jtuvz",
      "agent": "01a18330-85d7-7681-97b1-bbed4f0f08dd",
      "priority": "agent"
    },
    {
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0",
      "hash": "a949bf0423e69f6789b69aba4613cf22cbdc990d3a8ef69fdaf194d91660fd3c",
      "mode": "shared",
      "pid": 252045,
      "task": 125,
      "sinceMs": 1793213136884,
      "waitedSeconds": 4,
      "class": "unclassified",
      "rank": 0,
      "source": "registry",
      "position": 2,
      "queued": 6,
      "blockers": [
        "211237/120"
      ],
      "command": "f1f17c65ed4d13df7382a865f1624c25b1c583783b2fa9cd2d77f9ec14721e85 run repo:test-caller-environment --parallel=1",
      "session": "12974ae06f6b.a2m6c56sjvr",
      "agent": "01a18337-6fe3-7292-b687-d9a602297c07",
      "priority": "agent"
    },
    {
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0",
      "hash": "a949bf0423e69f6789b69aba4613cf22cbdc990d3a8ef69fdaf194d91660fd3c",
      "mode": "shared",
      "pid": 262605,
      "task": 123,
      "sinceMs": 1793213137324,
      "waitedSeconds": 4,
      "class": "unclassified",
      "rank": 0,
      "source": "registry",
      "position": 3,
      "queued": 6,
      "blockers": [
        "211237/120"
      ],
      "command": "091ff07d8e88f7f24bcbe9a0c7be8d2bdc16fdc3aec47571dfcace9b8cd061a4 run @teaching/architecture-course:test --parallel=1",
      "session": "12974ae06f6b.a2m6c56sjvr",
      "agent": "01a18337-6fe3-7292-b687-d9a602297c07",
      "priority": "agent"
    },
    {
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0",
      "hash": "a949bf0423e69f6789b69aba4613cf22cbdc990d3a8ef69fdaf194d91660fd3c",
      "mode": "shared",
      "pid": 229702,
      "task": 143,
      "sinceMs": 1793213138169,
      "waitedSeconds": 3,
      "class": "unclassified",
      "rank": 0,
      "source": "registry",
      "position": 6,
      "queued": 6,
      "blockers": [
        "211237/120"
      ],
      "command": "9a3bef7a8c50f4b35ae8296437f4762b483e0434a99c472e8855ddb8dc9fb7cd --tool=semio run @semio-tech/print:build-thesis --parallel=1",
      "session": "12974ae06f6b.a1lx59b1pwg",
      "agent": "01a18350-b1ec-7522-9d60-061b9baee7a0",
      "priority": "agent"
    },
    {
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0",
      "hash": "a949bf0423e69f6789b69aba4613cf22cbdc990d3a8ef69fdaf194d91660fd3c",
      "mode": "shared",
      "pid": 254696,
      "task": 129,
      "sinceMs": 1793213137870,
      "waitedSeconds": 3,
      "class": "unclassified",
      "rank": 0,
      "source": "registry",
      "position": 5,
      "queued": 6,
      "blockers": [
        "211237/120"
      ],
      "command": "f1f17c65ed4d13df7382a865f1624c25b1c583783b2fa9cd2d77f9ec14721e85 run @semio-tech/framework-os-mcp-rs:build --output=stream --parallel=1",
      "session": "12974ae06f6b.a2kddq7xlne",
      "agent": "codex-fad27566",
      "priority": "agent"
    }
  ],
  "waitFor": [
    {
      "from": "254696/129",
      "to": "211237/120",
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0"
    },
    {
      "from": "262605/123",
      "to": "211237/120",
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0"
    },
    {
      "from": "249683/123",
      "to": "211237/120",
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0"
    },
    {
      "from": "229702/143",
      "to": "211237/120",
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0"
    },
    {
      "from": "252045/125",
      "to": "211237/120",
      "resource": "graph-claim-slot:/home/deck/semio/.cache/b/g/m/generations:0"
    }
  ],
  "cycles": [],
  "violations": []
}
```
