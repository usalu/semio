# g1-m06 Round 31

Observation only; I start no setup command. Rewritten every 10 minutes, last at 2026-11-16T11:52:52Z. Machine: Mac mini, 16 GB memory, macOS 27.2.1. Whether a macOS permission dialog waits on screen: not measured (my shell cannot see the screen); the ready record below names a refusal if the tools met one.

## Ready Record (`origins/g1-m06/ready.json` in the store)

```
state: 
stage: 
attempts: 
exit: 
refusal: 
reason: 
ready: 
peakMegabytes: 
```

## Last 30 Lines of the Log of Stage 

```
```

## Load and Memory

```
12:53  up 1 day,  7:56, 2 users, load averages: 28.81 29.64 26.40
Pageouts: 1506293 

System-wide memory free percentage: 36%
```

Largest processes (`ps -axo pid,ppid,rss,pcpu,etime,command | sort -k3 -nr | head -25`, lines cut at 220 characters):

```
  593     1  31648   6.2 01-07:55:10 /System/Library/DriverExtensions/com.apple.DriverKit-AppleBCMWLAN.dext/com.apple.DriverKit-AppleBCMWLAN com.apple.bcmwlan 0x100000d13 com.apple.DriverKit-AppleBCMWLAN
  778     1  22944  14.9 01-07:54:57 /System/Library/CoreServices/loginwindow.app/Contents/MacOS/loginwindow console
 2186  2185  19344   1.7 01-07:51:12 /Users/ueli/Library/Application Support/Claude/claude-code/2.1.325/485fdac4332e/claude.app/Contents/MacOS/claude --output-format stream-json --verbose --input-format stream-json --eff
 1262     1  13440   1.7 01-07:52:26 /Applications/Claude.app/Contents/MacOS/Claude
  779     1  11456   4.6 01-07:54:57 /System/Library/PrivateFrameworks/SkyLight.framework/Resources/WindowServer -daemon
 9175     1  10944   0.9    02:12:19 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[
    1     0   9568   9.4 01-07:56:03 /sbin/launchd
 9427     1   8576   0.0    02:12:03 /System/Library/Frameworks/VideoToolbox.framework/Versions/A/XPCServices/VTDecoderXPCService.xpc/Contents/MacOS/VTDecoderXPCService
21372  3376   8432   0.8       54:45 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/.cache/f/code/203e01b3594f1a510964bfe95d
  401     1   7392   5.0 01-07:55:11 /usr/libexec/logd
 1260     1   6848   1.0 01-07:52:26 /Applications/GitKraken.app/Contents/MacOS/GitKraken
 1539  1260   5824   0.7 01-07:52:16 /Applications/GitKraken.app/Contents/Frameworks/GitKraken Helper (Renderer).app/Contents/MacOS/GitKraken Helper (Renderer) --type=renderer --user-data-dir=/Users/ueli/Library/Applicat
 1090     1   5824   1.3 01-07:52:31 /System/Library/ExtensionKit/Extensions/WallpaperAerialsExtension.appex/Contents/MacOS/WallpaperAerialsExtension -BSServiceDomains {"XPCService":{"Services":{"MainService":{}}}} -Laun
  442     1   5456  17.1 01-07:55:11 /usr/libexec/opendirectoryd
  466     1   5424   2.2 01-07:55:11 /usr/sbin/bluetoothd
  543     1   5408   2.5 01-07:55:10 /usr/libexec/airportd
 1264     1   5312   1.0 01-07:52:26 /System/Applications/Utilities/Activity Monitor.app/Contents/MacOS/Activity Monitor
56720     1   5280  10.0       01:06 /usr/libexec/mobileassetd
57206     1   5184   5.2       00:06 /System/Library/CoreServices/audioaccessoryd
 1270     1   5104   1.4 01-07:52:26 /System/Library/CoreServices/Finder.app/Contents/MacOS/Finder
  478     1   5104   1.7 01-07:55:10 /usr/libexec/runningboardd
57258     1   4832   2.4       00:01 /usr/libexec/modelmanagerd
57252     1   4736  10.8       00:01 /usr/sbin/cfprefsd agent
 3376  3360   4736   0.3 01-00:57:36 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Library/Application Support/semio/fleet/entry/boot.mjs r
29142     1   4720   1.0    17:21:58 /usr/sbin/coreaudiod
```

## History (One Line per Observation)

```
2026-11-16T03:11:19Z state=running stage=workspace attempts=0 exit=None ready=False peakMegabytes=1988 load=8.12 9.17 9.56 pressure_level=1
2026-11-16T03:21:19Z state=running stage=workspace attempts=0 exit=None ready=False peakMegabytes=1988 load=7.21 7.01 8.01 pressure_level=1
2026-11-16T03:31:20Z state=running stage=workspace attempts=0 exit=None ready=False peakMegabytes=1988 load=4.00 4.88 6.29 pressure_level=1
2026-11-16T03:41:21Z state=running stage=workspace attempts=0 exit=None ready=False peakMegabytes=6301 load=6.51 5.95 6.06 pressure_level=1
2026-11-16T03:51:22Z state=running stage=workspace attempts=0 exit=None ready=False peakMegabytes=6301 load=10.23 8.22 7.16 pressure_level=1
2026-11-16T04:01:23Z state=running stage=workspace attempts=0 exit=None ready=False peakMegabytes=6301 load=6.65 6.77 6.75 pressure_level=1
2026-11-16T04:11:24Z state=running stage=workspace attempts=0 exit=None ready=False peakMegabytes=6301 load=8.29 8.18 7.62 pressure_level=1
2026-11-16T04:21:26Z state=running stage=workspace attempts=0 exit=None ready=False peakMegabytes=6301 load=6.42 7.25 7.52 pressure_level=2
2026-11-16T04:31:27Z state=running stage=workspace attempts=1 exit=75 ready=False peakMegabytes=7539 load=16.07 12.37 10.04 pressure_level=1
2026-11-16T04:41:28Z state=running stage=workspace attempts=1 exit=75 ready=False peakMegabytes=7539 load=8.00 10.35 10.60 pressure_level=1
2026-11-16T04:51:29Z state=running stage=workspace attempts=1 exit=75 ready=False peakMegabytes=7539 load=7.50 7.32 8.71 pressure_level=1
2026-11-16T05:01:30Z state=running stage=workspace attempts=1 exit=75 ready=False peakMegabytes=7539 load=9.16 10.98 10.00 pressure_level=1
2026-11-16T05:11:31Z state=running stage=workspace attempts=1 exit=75 ready=False peakMegabytes=7539 load=7.87 10.64 10.61 pressure_level=1
2026-11-16T05:21:32Z state=running stage=workspace attempts=1 exit=75 ready=False peakMegabytes=7539 load=6.79 8.06 9.34 pressure_level=1
2026-11-16T05:31:33Z state=running stage=workspace attempts=1 exit=75 ready=False peakMegabytes=7539 load=11.15 12.57 11.33 pressure_level=1
2026-11-16T05:41:34Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=8.52 8.88 9.68 pressure_level=1
2026-11-16T05:51:35Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=13.41 12.04 10.70 pressure_level=1
2026-11-16T06:01:36Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=5.19 8.06 9.38 pressure_level=1
2026-11-16T06:11:37Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=9.15 9.58 9.51 pressure_level=1
2026-11-16T06:21:38Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=6.79 7.46 8.44 pressure_level=1
2026-11-16T06:31:39Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=4.95 6.66 7.77 pressure_level=1
2026-11-16T06:41:40Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=6.32 7.13 7.51 pressure_level=1
2026-11-16T06:51:41Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=8.80 8.15 7.71 pressure_level=1
2026-11-16T07:01:42Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=5.50 6.36 6.84 pressure_level=2
2026-11-16T07:11:43Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=8.32 7.38 7.22 pressure_level=1
2026-11-16T07:21:44Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=7.16 6.36 6.66 pressure_level=1
2026-11-16T07:31:45Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=6.87 6.57 6.54 pressure_level=1
2026-11-16T07:41:46Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=5.80 5.90 6.11 pressure_level=1
2026-11-16T07:51:47Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=4.71 4.89 5.52 pressure_level=1
2026-11-16T08:01:48Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=10.33 8.07 6.67 pressure_level=1
2026-11-16T08:11:49Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=9.57 6.90 6.54 pressure_level=1
2026-11-16T08:21:51Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=7.76 8.09 7.31 pressure_level=1
2026-11-16T08:31:52Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=6.51 6.31 6.64 pressure_level=1
2026-11-16T08:41:53Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=4.22 5.78 6.30 pressure_level=1
2026-11-16T08:51:54Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=5.79 6.46 6.46 pressure_level=1
2026-11-16T09:01:55Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=3.72 3.58 4.79 pressure_level=1
2026-11-16T09:11:55Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=5.02 4.51 4.65 pressure_level=1
2026-11-16T09:21:56Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=4.99 5.80 5.41 pressure_level=2
2026-11-16T09:31:57Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=7.00 6.23 5.79 pressure_level=2
2026-11-16T09:41:58Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=4.15 5.04 5.31 pressure_level=2
2026-11-16T10:03:39Z state=running stage=workspace attempts=2 exit=75 ready=False peakMegabytes=7539 load=1.67 1.75 1.68 pressure_level=2
2026-11-16T11:11:31Z state= stage= attempts= exit= ready= peakMegabytes= load=4.79 6.18 5.22 pressure_level=2
2026-11-16T11:21:32Z state= stage= attempts= exit= ready= peakMegabytes= load=28.73 22.48 13.88 pressure_level=2
2026-11-16T11:31:39Z state= stage= attempts= exit= ready= peakMegabytes= load=21.17 26.99 23.27 pressure_level=2
2026-11-16T11:42:08Z state= stage= attempts= exit= ready= peakMegabytes= load=15.79 19.46 20.76 pressure_level=4
2026-11-16T11:52:52Z state= stage= attempts= exit= ready= peakMegabytes= load=36.24 31.03 26.82 pressure_level=2
```
