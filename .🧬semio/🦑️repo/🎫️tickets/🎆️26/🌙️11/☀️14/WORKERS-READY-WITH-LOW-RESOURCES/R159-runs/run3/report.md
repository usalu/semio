# no verdict within 1516.1 s; last stage workspace, phase owners done

- slice of 1 owners requested, the read admitted 53
- stages replaced by `repo --help` (not exercised): native, repo-mcp, dependencies, tools
- peak footprint 5503 MB against the limit of 6553 MB
- owners inferred more than once across machines: 0
- leftover processes 0, leftover attached disk images 0
- load average at start 38.28
- lab processes that ran below normal priority (nice other than 0): 0
- parent event-loop lag, maximum - ms (limit 200 ms)
- processes of the machines busy in 21 percent of the samples
- seconds per activity: {"-":2,"build:source-admission":22,"build:implementation-source":24,"tools":86,"capacity":2,"build:selected-compile-closure":2,"build:command-source-imports":38,"build:selected-normalization":10,"build:command-source-packages":10,"build:selected-compile-imports":6,"build:selected-source-inputs":26,"build:projects":20,"build:project-assembly":6,"build:modules":4,"build:graph":34,"build:portable-inputs":962}
- selected graph publications sealed on the machines: 1: 197b3fa666cd486704afa6b2486e41d34937781dbf655c65569ad69a0940aa24.json
- owners of the read per machine (maximum seen: present / fetched from the fleet / inferred here of total): {"slice-lab-4f7b-m1":"0 / 0 / 34 of 53"}
- seconds each machine took from its own start to ready: {}
- disk images the lab detached after the machines were stopped: 6
- files of the frozen view that changed during the run (must be 0): 0
- frozen view: 167696 files (1839 MB logical, cloned in 339 s without copying bytes where the file system clones); 19 files of the live tree changed since the freeze and stay invisible to the machines
- not exercised by a selected read yet: assembly, validation, seal and publish of a slice generation, and a target run on the slice
- files below the live state of the checkout that the machines held open: 0
- seconds per stage: {}
- seconds per phase: {"start":94,"taken":40,"owners done":996}

| t (s) | machine | stage | state | phase | done | total | owners | procs | busy | footprint MB | cpu % | loop lag ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 2.3 | slice-lab-4f7b-m1 | repo-mcp | running | - | - | - | {} | 1 | 0 | 50 | 0.5 | - |
| 4.6 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 4 | 2 | 289 | 101.7 | - |
| 6.8 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 4 | 2 | 317 | 12.9 | - |
| 8.8 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 4 | 2 | 329 | 64.7 | - |
| 11 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 4 | 2 | 527 | 48.4 | - |
| 13.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 3 | 722 | 55.2 | - |
| 15.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 4 | 949 | 81.1 | - |
| 17.5 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 0 | 950 | 0.8 | - |
| 19.6 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 950 | 1.7 | - |
| 21.7 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 950 | 2.1 | - |
| 24.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 951 | 2.1 | - |
| 26.3 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 958 | 4.1 | - |
| 28.4 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 958 | 1.4 | - |
| 30.5 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 960 | 1.7 | - |
| 32.6 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 960 | 1.2 | - |
| 34.6 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 961 | 1.6 | - |
| 36.8 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 962 | 2.7 | - |
| 38.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1008 | 29 | - |
| 40.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1012 | 9.3 | - |
| 43.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1013 | 7.3 | - |
| 45.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1013 | 7.5 | - |
| 47.2 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1054 | 6.2 | - |
| 49.3 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1068 | 25.8 | - |
| 51.4 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1094 | 15.9 | - |
| 53.6 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1095 | 12.2 | - |
| 56 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1095 | 9.5 | - |
| 58.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1096 | 9.7 | - |
| 60.3 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1096 | 6.7 | - |
| 62.7 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1096 | 11.2 | - |
| 64.8 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1097 | 8.9 | - |
| 66.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1098 | 14.3 | - |
| 69.2 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1100 | 10.3 | - |
| 71.3 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1101 | 26 | - |
| 73.4 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1104 | 8 | - |
| 75.7 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1109 | 11.7 | - |
| 77.8 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1109 | 8.5 | - |
| 79.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1110 | 11.7 | - |
| 82.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1110 | 4 | - |
| 84.2 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 3 | 1128 | 13.6 | - |
| 86.2 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 2 | 1289 | 73.7 | - |
| 88.4 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 1 | 1511 | 54.5 | - |
| 90.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 2 | 1515 | 24 | - |
| 92.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 3 | 1558 | 73.8 | - |
| 95.3 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 1 | 1558 | 11.6 | - |
| 97.3 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 1 | 1558 | 55.6 | - |
| 99.4 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 1 | 1563 | 13.1 | - |
| 101.8 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 1 | 1591 | 94 | - |
| 103.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 7 | 1 | 1633 | 7.7 | - |
| 105.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1111 | 4 | - |
| 108.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 0 | 1111 | 0 | - |
| 110.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1112 | 23.3 | - |
| 112.2 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1115 | 16 | - |
| 114.3 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1115 | 42.9 | - |
| 116.6 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1116 | 15.6 | - |
| 118.6 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1116 | 47.7 | - |
| 120.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1116 | 25.7 | - |
| 122.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 2 | 1133 | 78 | - |
| 125 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1133 | 2.3 | - |
| 127.8 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1133 | 1.5 | - |
| 129.9 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1133 | 2 | - |
| 132.1 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 5 | 1 | 1134 | 7.1 | - |
| 134.7 | slice-lab-4f7b-m1 | workspace | running | - | - | - | {} | 6 | 3 | 1229 | 73.8 | - |
| 136.8 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 2 | 1321 | 36.3 | - |
| 138.8 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1334 | 69.2 | - |
| 141 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1410 | 30.9 | - |
| 143.1 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 0 | 1410 | 0.6 | - |
| 145.1 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1411 | 2.6 | - |
| 147.5 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1413 | 18.4 | - |
| 149.6 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1559 | 68.8 | - |
| 151.6 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1567 | 84.4 | - |
| 153.9 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1625 | 137.9 | - |
| 156 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1669 | 70.9 | - |
| 158 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1711 | 79.3 | - |
| 160.4 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1765 | 57.6 | - |
| 162.5 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1927 | 41.6 | - |
| 164.8 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 2 | 1948 | 16.7 | - |
| 167.6 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 3 | 1952 | 34.8 | - |
| 169.7 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 3 | 1990 | 34.8 | - |
| 171.8 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 1993 | 56.2 | - |
| 173.9 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2010 | 95.1 | - |
| 176 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2017 | 42.6 | - |
| 178.1 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2033 | 68.5 | - |
| 180.4 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2038 | 54.2 | - |
| 182.4 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2044 | 45.9 | - |
| 184.5 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2052 | 32.8 | - |
| 186.7 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 2 | 2053 | 23.9 | - |
| 188.8 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 2 | 2055 | 53 | - |
| 191 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 2 | 2055 | 37.1 | - |
| 193.5 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2055 | 19.9 | - |
| 195.6 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2056 | 25.9 | - |
| 198.2 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2056 | 10.4 | - |
| 201.1 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 2 | 2056 | 22.7 | - |
| 203.1 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2064 | 55.3 | - |
| 205.2 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2065 | 89.7 | - |
| 207.4 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2079 | 85.8 | - |
| 209.6 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2161 | 57.8 | - |
| 211.6 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2162 | 16.4 | - |
| 213.8 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 2 | 2166 | 80.4 | - |
| 215.9 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2210 | 88.4 | - |
| 218 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2233 | 74.1 | - |
| 220.6 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2236 | 58.9 | - |
| 222.7 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 1 | 2415 | 64.9 | - |
| 224.7 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {} | 6 | 2 | 2417 | 65.8 | - |
| 227 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2418 | 51.7 | - |
| 229.1 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2429 | 67.4 | - |
| 231.1 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2430 | 26.1 | - |
| 234 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2430 | 33.6 | - |
| 236.3 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2431 | 21.4 | - |
| 238.4 | slice-lab-4f7b-m1 | workspace | running | start | 0 | 0 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2431 | 65.9 | - |
| 240.5 | slice-lab-4f7b-m1 | workspace | running | taken | 25 | 53 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2434 | 71.4 | - |
| 243.1 | slice-lab-4f7b-m1 | workspace | running | taken | 25 | 53 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2434 | 35 | - |
| 245.1 | slice-lab-4f7b-m1 | workspace | running | taken | 25 | 53 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2442 | 59.1 | - |
| 247.3 | slice-lab-4f7b-m1 | workspace | running | taken | 26 | 53 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2443 | 87.2 | - |
| 249.3 | slice-lab-4f7b-m1 | workspace | running | taken | 26 | 53 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2447 | 95.2 | - |
| 251.4 | slice-lab-4f7b-m1 | workspace | running | taken | 26 | 53 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2461 | 89.3 | - |
| 253.7 | slice-lab-4f7b-m1 | workspace | running | taken | 26 | 53 | {"total":53,"present":0,"fetched":0,"here":18,"stale":0,"running":3,"wanted":0,"done":18,"missing":35} | 6 | 1 | 2484 | 21 | - |
| 255.7 | slice-lab-4f7b-m1 | workspace | running | taken | 26 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2514 | 58.2 | - |
| 257.8 | slice-lab-4f7b-m1 | workspace | running | taken | 26 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2514 | 85.5 | - |
| 260 | slice-lab-4f7b-m1 | workspace | running | taken | 27 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2531 | 94.6 | - |
| 262.2 | slice-lab-4f7b-m1 | workspace | running | taken | 27 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2533 | 46 | - |
| 264.2 | slice-lab-4f7b-m1 | workspace | running | taken | 27 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2533 | 88.3 | - |
| 266.6 | slice-lab-4f7b-m1 | workspace | running | taken | 27 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2533 | 21.1 | - |
| 268.7 | slice-lab-4f7b-m1 | workspace | running | taken | 27 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2533 | 40.5 | - |
| 270.9 | slice-lab-4f7b-m1 | workspace | running | taken | 49 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2533 | 24.1 | - |
| 273.2 | slice-lab-4f7b-m1 | workspace | running | taken | 49 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2536 | 57.5 | - |
| 275.3 | slice-lab-4f7b-m1 | workspace | running | taken | 49 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2566 | 32.8 | - |
| 277.4 | slice-lab-4f7b-m1 | workspace | running | taken | 50 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2566 | 76 | - |
| 279.5 | slice-lab-4f7b-m1 | workspace | running | taken | 50 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2590 | 155.1 | - |
| 281.6 | slice-lab-4f7b-m1 | workspace | running | taken | 50 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2644 | 128.9 | - |
| 283.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {"total":53,"present":0,"fetched":0,"here":34,"stale":0,"running":4,"wanted":0,"done":34,"missing":19} | 6 | 1 | 2810 | 101.5 | - |
| 285.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2821 | 85.9 | - |
| 288 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 2821 | 36.8 | - |
| 290.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 2821 | 74.5 | - |
| 292.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2821 | 35.4 | - |
| 294.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2822 | 62.4 | - |
| 296.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2822 | 79.6 | - |
| 298.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2823 | 36.2 | - |
| 300.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2882 | 53.7 | - |
| 303.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2882 | 12.4 | - |
| 306.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2882 | 17.2 | - |
| 308.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2882 | 12.2 | - |
| 310.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 2882 | 31.4 | - |
| 313.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 2882 | 23.9 | - |
| 315.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 2882 | 35 | - |
| 317.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 2882 | 72.8 | - |
| 319.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 2882 | 26.3 | - |
| 321.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2903 | 78.4 | - |
| 323.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2903 | 34.7 | - |
| 326.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2903 | 74.4 | - |
| 328.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2903 | 75.2 | - |
| 330.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 2903 | 31.2 | - |
| 332.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0 | - |
| 334.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0.9 | - |
| 336.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0 | - |
| 339.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0 | - |
| 341.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0 | - |
| 343.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0.6 | - |
| 345.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0 | - |
| 348 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0 | - |
| 350 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 2903 | 0 | - |
| 352.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 0 | 1190 | 0 | - |
| 354.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 1391 | 308.8 | - |
| 356.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 1575 | 310.7 | - |
| 358.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 1677 | 56.4 | - |
| 360.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 1678 | 213.6 | - |
| 362.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 1898 | 191.3 | - |
| 365.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2048 | 97.5 | - |
| 367.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2111 | 24.8 | - |
| 369.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 2 | 2141 | 127.4 | - |
| 371.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2206 | 39.3 | - |
| 373.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2223 | 130.7 | - |
| 375.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2235 | 121.2 | - |
| 378.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2235 | 78.8 | - |
| 380.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2304 | 129.1 | - |
| 382.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2343 | 52.9 | - |
| 385 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2347 | 176.9 | - |
| 387.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2421 | 43.9 | - |
| 389.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2469 | 192 | - |
| 391.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2641 | 44.1 | - |
| 393.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2645 | 81.5 | - |
| 395.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2688 | 94 | - |
| 397.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 1 | 2704 | 16 | - |
| 399.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 2 | 2771 | 169 | - |
| 402 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 2 | 2834 | 60.7 | - |
| 404.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 5 | 2 | 3027 | 93.9 | - |
| 406.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 3439 | 30 | - |
| 408.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 3524 | 84 | - |
| 410.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3656 | 1.6 | - |
| 413.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 3657 | 0.4 | - |
| 415.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 3702 | 71.4 | - |
| 417.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 3702 | 0.6 | - |
| 419.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 3702 | 1.3 | - |
| 421.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 0 | 3702 | 1.1 | - |
| 423.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3703 | 34.7 | - |
| 425.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3719 | 66.9 | - |
| 428.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3746 | 43.4 | - |
| 431.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3801 | 70.2 | - |
| 433.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3805 | 43.9 | - |
| 435.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 3843 | 76.1 | - |
| 437.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3917 | 49 | - |
| 439.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3943 | 80.5 | - |
| 442.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3974 | 29.8 | - |
| 444.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 3979 | 83 | - |
| 447.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4010 | 27.7 | - |
| 449.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4010 | 14.1 | - |
| 452.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4014 | 29 | - |
| 454.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4016 | 33.3 | - |
| 456.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4076 | 40.8 | - |
| 458.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4102 | 25.9 | - |
| 460.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4161 | 30.9 | - |
| 462.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4206 | 27.3 | - |
| 465.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4302 | 88.2 | - |
| 467.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4321 | 34.1 | - |
| 469.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4329 | 50.4 | - |
| 471.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4332 | 61.6 | - |
| 473.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4332 | 42 | - |
| 476 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 59.2 | - |
| 478.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 8.4 | - |
| 480.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4368 | 45.7 | - |
| 483.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4367 | 17.4 | - |
| 486.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 39 | - |
| 488.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 18.4 | - |
| 490.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4368 | 38.8 | - |
| 492.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 33.6 | - |
| 494.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4368 | 31.8 | - |
| 496.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 52.5 | - |
| 499 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 27.1 | - |
| 501 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 39.1 | - |
| 503.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 21.1 | - |
| 505.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 35.6 | - |
| 507.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4368 | 25.9 | - |
| 509.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4368 | 39.4 | - |
| 512.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4382 | 44.4 | - |
| 514.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4427 | 30.1 | - |
| 516.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4474 | 31.6 | - |
| 519.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4474 | 12.5 | - |
| 521.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4475 | 26.2 | - |
| 523.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4486 | 23.8 | - |
| 526 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4486 | 30.2 | - |
| 528.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4486 | 25.9 | - |
| 530.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4486 | 38.8 | - |
| 532.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4489 | 69.2 | - |
| 534.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4607 | 45.9 | - |
| 536.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4607 | 93.6 | - |
| 539.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4607 | 32.8 | - |
| 541.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 94.6 | - |
| 543.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 30.3 | - |
| 545.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4608 | 92.9 | - |
| 548.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 55.6 | - |
| 550.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 35.3 | - |
| 552.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 29.6 | - |
| 555.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 19.4 | - |
| 557.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 91 | - |
| 559.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4608 | 21.4 | - |
| 561.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 95.4 | - |
| 564 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 21.2 | - |
| 566.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4608 | 95.4 | - |
| 568.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4608 | 31.9 | - |
| 570.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4608 | 85.9 | - |
| 573.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4609 | 93.6 | - |
| 575.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4609 | 38 | - |
| 577.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4609 | 91.6 | - |
| 579.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4609 | 37.9 | - |
| 581.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4615 | 89 | - |
| 584.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4615 | 32.2 | - |
| 586.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4615 | 87 | - |
| 589 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4615 | 18 | - |
| 591.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4615 | 22.2 | - |
| 593.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4615 | 91.9 | - |
| 595.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4615 | 41.2 | - |
| 597.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4615 | 90.7 | - |
| 600 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4615 | 25.3 | - |
| 602 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4640 | 92 | - |
| 604.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4640 | 42.1 | - |
| 606.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4640 | 86.4 | - |
| 608.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4640 | 95.6 | - |
| 610.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4640 | 25.8 | - |
| 612.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4640 | 93.2 | - |
| 615 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4664 | 18.7 | - |
| 617.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4664 | 63.4 | - |
| 619.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4664 | 77.8 | - |
| 621.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4666 | 18.5 | - |
| 624 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4751 | 30.6 | - |
| 626.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4801 | 28.4 | - |
| 628.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4837 | 37.1 | - |
| 630.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4869 | 26.1 | - |
| 633.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 4916 | 28.8 | - |
| 635.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 3 | 4960 | 21.7 | - |
| 637.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4985 | 21.7 | - |
| 640 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5016 | 19.9 | - |
| 642.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5064 | 29.9 | - |
| 645 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5095 | 35.9 | - |
| 647.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5131 | 31.3 | - |
| 649.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5164 | 20.7 | - |
| 651.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5174 | 46.8 | - |
| 653.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5206 | 33 | - |
| 656.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 5251 | 21.5 | - |
| 659.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5314 | 26.5 | - |
| 661.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5314 | 10.1 | - |
| 663.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5318 | 37.5 | - |
| 665.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5318 | 21.3 | - |
| 669.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5320 | 32.4 | - |
| 671.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5322 | 25.7 | - |
| 673.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5322 | 16 | - |
| 675.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5322 | 18.1 | - |
| 678 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5322 | 4.7 | - |
| 680.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5322 | 12.5 | - |
| 683.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 5322 | 14.8 | - |
| 685.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5322 | 7 | - |
| 687.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5323 | 14.3 | - |
| 690 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5323 | 4.9 | - |
| 693.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4790 | 23 | - |
| 695.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4790 | 16 | - |
| 698 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4790 | 34.4 | - |
| 700.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4790 | 28.5 | - |
| 702.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4791 | 32.4 | - |
| 705.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4795 | 27.9 | - |
| 707.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4795 | 33.8 | - |
| 709.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4795 | 28.3 | - |
| 711.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4795 | 94.6 | - |
| 714 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4795 | 26.7 | - |
| 717.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4795 | 74.3 | - |
| 719.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4795 | 37 | - |
| 721.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4824 | 93.1 | - |
| 723.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4824 | 20.4 | - |
| 726.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4824 | 93 | - |
| 729 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4825 | 8.6 | - |
| 731.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4825 | 65.9 | - |
| 733.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4825 | 97.8 | - |
| 735.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4825 | 16.4 | - |
| 737.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4825 | 92.4 | - |
| 740.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4825 | 39 | - |
| 742.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4825 | 92.9 | - |
| 744.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4825 | 23.9 | - |
| 747.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4825 | 93.4 | - |
| 749.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4825 | 32.2 | - |
| 752.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4827 | 96.1 | - |
| 754.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4827 | 57.1 | - |
| 756.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4827 | 94.1 | - |
| 758.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4827 | 48.2 | - |
| 761.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4828 | 89 | - |
| 763.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 4856 | 5.9 | - |
| 766.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4892 | 6.5 | - |
| 768.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4900 | 13.5 | - |
| 770.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4900 | 5.8 | - |
| 772.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4900 | 7.2 | - |
| 775.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4901 | 6.1 | - |
| 777.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4902 | 10.6 | - |
| 779.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4902 | 10.1 | - |
| 782 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4903 | 14.8 | - |
| 784 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4903 | 13.7 | - |
| 787.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4829 | 35.8 | - |
| 789.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 94 | - |
| 791.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 48.6 | - |
| 793.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 44.6 | - |
| 796.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 32.6 | - |
| 799.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 16.2 | - |
| 801.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 11.8 | - |
| 804.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 94.3 | - |
| 806.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 48.4 | - |
| 808.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 97.4 | - |
| 811.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 24.1 | - |
| 813.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4830 | 98.1 | - |
| 815.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4830 | 52.7 | - |
| 818 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4831 | 94 | - |
| 820.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4831 | 90.7 | - |
| 823.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4831 | 90.4 | - |
| 825.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4831 | 40.1 | - |
| 827.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4831 | 90.2 | - |
| 830 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4831 | 41.8 | - |
| 832.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4831 | 41.7 | - |
| 835.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4831 | 10 | - |
| 838.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4831 | 13.4 | - |
| 840.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4832 | 47.7 | - |
| 842.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4832 | 44.2 | - |
| 844.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4832 | 94.1 | - |
| 847.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4832 | 78.5 | - |
| 849.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4832 | 93.4 | - |
| 852 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4832 | 16.6 | - |
| 854.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4832 | 95.3 | - |
| 856.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4832 | 92.7 | - |
| 860 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 4859 | 15.1 | - |
| 862.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 4859 | 6.7 | - |
| 864.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 4859 | 16.2 | - |
| 866.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4859 | 4.4 | - |
| 869.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4859 | 2.5 | - |
| 874.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 4860 | 11.4 | - |
| 876.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4860 | 6.6 | - |
| 878.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4860 | 4.2 | - |
| 880.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4860 | 5.1 | - |
| 882.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4860 | 8.7 | - |
| 886.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4860 | 6.7 | - |
| 888.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 3 | 4833 | 47.7 | - |
| 890.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4833 | 43.7 | - |
| 892.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 92.5 | - |
| 895 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 95.2 | - |
| 897.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4833 | 19.4 | - |
| 900.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 18.8 | - |
| 902.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 95.5 | - |
| 904.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 16.3 | - |
| 907.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 75.3 | - |
| 910 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 14.7 | - |
| 912.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 92.2 | - |
| 914.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4833 | 48.1 | - |
| 916.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 93.5 | - |
| 918.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 88 | - |
| 922.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 90.6 | - |
| 925.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 41.7 | - |
| 927.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4833 | 95.1 | - |
| 929.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4833 | 50.2 | - |
| 931.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 66.9 | - |
| 934.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 32 | - |
| 936.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 65.2 | - |
| 939.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4833 | 73 | - |
| 941.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 12.2 | - |
| 943.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 95.5 | - |
| 947.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 4833 | 94.7 | - |
| 949.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 47.4 | - |
| 951.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4833 | 80.5 | - |
| 953.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 4847 | 80.8 | - |
| 956.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 5.1 | - |
| 959.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 4.1 | - |
| 961.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 6.6 | - |
| 963.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 5.7 | - |
| 965.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 4 | - |
| 967.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 8.4 | - |
| 970.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 3.6 | - |
| 973.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 4861 | 11 | - |
| 975.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 3.9 | - |
| 977.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 8.7 | - |
| 979.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 4861 | 8.1 | - |
| 983.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4891 | 39 | - |
| 985.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4891 | 36.9 | - |
| 988.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4891 | 29 | - |
| 990.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4891 | 15.1 | - |
| 992.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4891 | 22.7 | - |
| 996.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4891 | 33.6 | - |
| 998.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 4891 | 52.9 | - |
| 1000.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5025 | 70 | - |
| 1002.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5026 | 47.8 | - |
| 1004.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5038 | 89.2 | - |
| 1008.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5039 | 25 | - |
| 1010.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5040 | 15.3 | - |
| 1013.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5043 | 16.5 | - |
| 1015.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5043 | 83.8 | - |
| 1017.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5043 | 23.4 | - |
| 1021 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5043 | 93.3 | - |
| 1023.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 19.4 | - |
| 1025.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 96.1 | - |
| 1027.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 19.4 | - |
| 1029.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 94.6 | - |
| 1032.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 21.2 | - |
| 1034.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 93.2 | - |
| 1036.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 54.4 | - |
| 1039.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5044 | 97.3 | - |
| 1041.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 30.4 | - |
| 1046 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5044 | 26.4 | - |
| 1048.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 23.9 | - |
| 1050.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 88 | - |
| 1052.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 54.7 | - |
| 1055 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 94.8 | - |
| 1057.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 28.2 | - |
| 1060 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 97.5 | - |
| 1062.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 65.8 | - |
| 1064.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 74.2 | - |
| 1066.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 99.2 | - |
| 1070 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 92.4 | - |
| 1072.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 70.7 | - |
| 1074.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 57.5 | - |
| 1076.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 40.5 | - |
| 1078.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 20.7 | - |
| 1082.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 25.1 | - |
| 1085.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 3 | 5045 | 57.3 | - |
| 1087.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 49.3 | - |
| 1089.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 90.1 | - |
| 1092 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 96.6 | - |
| 1096 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 52.1 | - |
| 1098.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 12.4 | - |
| 1100.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 89 | - |
| 1103 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 40.4 | - |
| 1105.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 5063 | 15.5 | - |
| 1108.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 5063 | 5 | - |
| 1110.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5063 | 8.2 | - |
| 1112.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5064 | 3.7 | - |
| 1115.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5063 | 3 | - |
| 1117.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5063 | 3.6 | - |
| 1123.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5064 | 5 | - |
| 1125.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5064 | 7.3 | - |
| 1127.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5064 | 8.7 | - |
| 1129.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5064 | 6.7 | - |
| 1131.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 94 | - |
| 1135.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 77 | - |
| 1137.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 95.1 | - |
| 1139.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 22 | - |
| 1141.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 102 | - |
| 1143.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 34.2 | - |
| 1147.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 51.3 | - |
| 1149.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 17.2 | - |
| 1151.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 31 | - |
| 1154.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 13.6 | - |
| 1156.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 77.4 | - |
| 1159.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 34.8 | - |
| 1161.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 96.7 | - |
| 1163.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 51.2 | - |
| 1166 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5045 | 90.2 | - |
| 1168.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5045 | 88.3 | - |
| 1172.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 2 | 5072 | 49.7 | - |
| 1174.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5072 | 4.1 | - |
| 1177.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5072 | 5.8 | - |
| 1179.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5072 | 5.8 | - |
| 1181.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 7 | 1 | 5072 | 5.5 | - |
| 1204.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 29.5 | - |
| 1206.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 44.9 | - |
| 1208.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 28.3 | - |
| 1211.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 42 | - |
| 1213.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 39.1 | - |
| 1217.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 17.6 | - |
| 1220 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 57.3 | - |
| 1222.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 30.9 | - |
| 1224.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5087 | 68.7 | - |
| 1226.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 28.4 | - |
| 1230.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 92.1 | - |
| 1232.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 33.3 | - |
| 1234.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 93.5 | - |
| 1236.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5087 | 29.7 | - |
| 1238.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 96 | - |
| 1242 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5088 | 31.6 | - |
| 1244.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 95 | - |
| 1246.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 30.4 | - |
| 1248.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5088 | 94.7 | - |
| 1250.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5088 | 37.8 | - |
| 1256 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5088 | 36.6 | - |
| 1258.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 8.5 | - |
| 1260.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 61.7 | - |
| 1262.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 17.8 | - |
| 1264.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 94 | - |
| 1268.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 69.9 | - |
| 1270.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 52.2 | - |
| 1273.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5088 | 90.8 | - |
| 1275.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5088 | 89.2 | - |
| 1277.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 43 | - |
| 1280.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5088 | 89.1 | - |
| 1282.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 52.3 | - |
| 1285 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 85.9 | - |
| 1287.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5088 | 70.1 | - |
| 1289.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5088 | 24.7 | - |
| 1292.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5089 | 36.9 | - |
| 1295.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5090 | 54.9 | - |
| 1297.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5090 | 51.6 | - |
| 1299.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5115 | 50.8 | - |
| 1301.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5124 | 92.4 | - |
| 1306.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5124 | 82.8 | - |
| 1308.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5124 | 43.9 | - |
| 1310.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5359 | 69.8 | - |
| 1313 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5359 | 78.5 | - |
| 1315.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5385 | 88.6 | - |
| 1318.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5388 | 53.9 | - |
| 1320.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5389 | 91.8 | - |
| 1322.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5490 | 37.1 | - |
| 1324.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5490 | 22.9 | - |
| 1326.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5490 | 42.2 | - |
| 1332.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5490 | 37.7 | - |
| 1334.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5490 | 28.1 | - |
| 1336.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5497 | 65.2 | - |
| 1338.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5497 | 49.4 | - |
| 1341 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5497 | 82.7 | - |
| 1344.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 54.5 | - |
| 1346.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5498 | 61.9 | - |
| 1348.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 34.3 | - |
| 1350.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 42.2 | - |
| 1352.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5498 | 60.1 | - |
| 1357.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 55 | - |
| 1359.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 23.1 | - |
| 1361.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 50.9 | - |
| 1364.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 32.3 | - |
| 1366.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 52.8 | - |
| 1371.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 58.4 | - |
| 1373.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 50.6 | - |
| 1375.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 37.3 | - |
| 1377.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 37.7 | - |
| 1379.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 29.4 | - |
| 1384 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5498 | 27.7 | - |
| 1386.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5498 | 32.4 | - |
| 1388.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 36.7 | - |
| 1390.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 26.8 | - |
| 1392.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 91.9 | - |
| 1397.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 52.8 | - |
| 1399.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 47.5 | - |
| 1401.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 18 | - |
| 1403.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 40.7 | - |
| 1405.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5498 | 39.5 | - |
| 1409.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 28.6 | - |
| 1411.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 39 | - |
| 1413.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5498 | 38.7 | - |
| 1415.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5499 | 28.8 | - |
| 1418.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 90.2 | - |
| 1421.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 40.2 | - |
| 1423.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 61.5 | - |
| 1425.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5499 | 44.2 | - |
| 1428.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 54.4 | - |
| 1430.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 22.8 | - |
| 1435.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 13.5 | - |
| 1437.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 20.2 | - |
| 1439.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 31.5 | - |
| 1442.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 39.7 | - |
| 1444.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5499 | 4.9 | - |
| 1447.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 28.8 | - |
| 1449.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 39.8 | - |
| 1451.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 27.4 | - |
| 1453.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 3 | 5499 | 42.3 | - |
| 1455.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5500 | 27.5 | - |
| 1459.8 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5500 | 38.4 | - |
| 1461.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5499 | 28.3 | - |
| 1464.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5500 | 30.1 | - |
| 1466.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5500 | 28.9 | - |
| 1468.7 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5500 | 27.5 | - |
| 1474.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5500 | 38.9 | - |
| 1477 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5502 | 72.7 | - |
| 1479.1 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5502 | 35.9 | - |
| 1481.2 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5502 | 35.6 | - |
| 1483.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5502 | 32.9 | - |
| 1486.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5502 | 28 | - |
| 1489.3 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5502 | 39.6 | - |
| 1491.4 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5502 | 31.5 | - |
| 1493.5 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 2 | 5502 | 39.8 | - |
| 1495.9 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5503 | 77.1 | - |
| 1500.6 | slice-lab-4f7b-m1 | workspace | running | owners done | 53 | 53 | {} | 6 | 1 | 5503 | 41.7 | - |

## home

- tools prepared for this slice: 
- paths outside the lab seam that changed during the run: 17: /Users/ueli/Documents/semio/.cache/boot-recovery/26d965e295a24b3a51d4095d22775fb08529c921998c5234b017ac091a8048fb.jsonl.stamp, /Users/ueli/Documents/semio/.cache/f, /Users/ueli/Documents/semio/.cache/f/boot, /Users/ueli/Documents/semio/.cache/f/cache-facts.json, /Users/ueli/Documents/semio/.cache/f/daemon.lock, /Users/ueli/Documents/semio/.cache/f/graph-server, /Users/ueli/Documents/semio/.cache/f/health.json, /Users/ueli/Documents/semio/.cache/f/mirror-state.json, /Users/ueli/Documents/semio/.cache/f/owner-pool-children, /Users/ueli/Documents/semio/.cache/f/owner-pool-fresh, /Users/ueli/Documents/semio/.cache/f/owner-pool-waves, /Users/ueli/Documents/semio/.cache/f/owner-pool.pid, ... (+5)
- downloads the stages tried and the lab refused: 0

| seed | method | seconds | machine |
| --- | --- | --- | --- |
| launcher | clonefile | 0 | slice-lab-4f7b-m1 |
| repo | clonefile | 0 | slice-lab-4f7b-m1 |
| runtimes | clonefile | 0 | slice-lab-4f7b-m1 |

slice-lab-4f7b-m1

| stage | seconds | verdict |
| --- | --- | --- |
| native | - | not reached |
| repo-mcp | 2 | passed (seen in samples) |
| dependencies | - | not reached |
| tools | - | not reached |
| workspace | 1252 | running |

