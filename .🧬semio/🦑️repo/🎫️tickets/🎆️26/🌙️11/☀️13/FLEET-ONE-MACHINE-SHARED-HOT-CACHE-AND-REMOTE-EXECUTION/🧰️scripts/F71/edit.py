#!/usr/bin/env python3
"""F71: anchored, atomic edits (temp + rename) that give every client-to-broker admission request a deadline, heartbeat and verdict."""
import os, sys

ROOT = "/Users/ueli/Documents/semio/🧰️framework/"
ADM = ROOT + "🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/"
UC = ROOT + "🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/"


def edit(path, pairs):
    text = open(path, encoding="utf-8").read()
    for old, new in pairs:
        if text.count(old) != 1:
            sys.exit(f"anchor not unique ({text.count(old)}) in {path}: {old[:80]!r}")
        text = text.replace(old, new)
    tmp = path + ".f71tmp"
    with open(tmp, "w", encoding="utf-8") as handle:
        handle.write(text)
    os.replace(tmp, path)


which = sys.argv[1]

if which == "schema":
    edit(ADM + "🧬️schema/🔣️.json", [
        ('          "progressMilliseconds": 30000\n        },\n        "unitCeiling": {',
         '          "progressMilliseconds": 30000\n        },\n'
         '        "bounds": {\n'
         '          "description": "Every client-to-broker request ends in a verdict, never in silence: a unit that waits in the broker queue is told so every `heartbeatMilliseconds` (the wrapper treats `silenceMilliseconds` without any line as a lost broker, drops the connection, and sends the request again, at most `attempts` times with a growing pause, so a request survives a broker that restarted); estimating the memory of a unit is bounded by `predictMilliseconds`; a grant growth by its own wait plus `growthGraceMilliseconds`; a unit that waits with nothing running for `stallMilliseconds` runs alone beside the budget it cannot get; a unit that waits longer than `queueMilliseconds` is refused. The Rust scheduler keeps `silenceMilliseconds` and `attempts` as constants and a test compares them.",\n'
         '          "heartbeatMilliseconds": 30000,\n'
         '          "silenceMilliseconds": 180000,\n'
         '          "attempts": 5,\n'
         '          "predictMilliseconds": 120000,\n'
         '          "growthGraceMilliseconds": 30000,\n'
         '          "stallMilliseconds": 90000,\n'
         '          "queueMilliseconds": 2700000\n'
         '        },\n        "unitCeiling": {'),
        ('            "row": "[capacity]   unit wait, pid {pid}: {text}"',
         '            "stalled": "unit {unit} waited {seconds} s with nothing running although it needs about {predicted} MB and the budget is {budget} MB: it runs alone now so that the build cannot stand still",\n'
         '            "expired": "unit {unit} waited {seconds} s in the broker queue, longer than the bound of {minutes} min: it is refused so that the build ends instead of waiting for ever; retry on a quieter host or a fleet worker",\n'
         '            "late": "unit {unit}: the broker did not finish estimating its memory within {seconds} s, the request is dropped and the compiler asks again",\n'
         '            "row": "[capacity]   unit wait, pid {pid}: {text}"'),
        ('            "row": "[capacity]   Einheit wartet, pid {pid}: {text}"',
         '            "stalled": "Einheit {unit} wartete {seconds} s, obwohl nichts lief: sie braucht etwa {predicted} MB und das Budget beträgt {budget} MB; sie läuft jetzt allein, damit der Bau nicht stillsteht",\n'
         '            "expired": "Einheit {unit} wartete {seconds} s in der Warteschlange des Brokers, länger als die Grenze von {minutes} min: sie wird abgelehnt, damit der Bau endet, statt ewig zu warten; auf einem ruhigeren Host oder einem Flotten-Worker erneut versuchen",\n'
         '            "late": "Einheit {unit}: der Broker hat die Speicherschätzung nicht innerhalb von {seconds} s abgeschlossen, die Anfrage wird verworfen und der Compiler fragt erneut",\n'
         '            "row": "[capacity]   Einheit wartet, pid {pid}: {text}"'),
    ])

if which == "broker":
    edit(ADM + "🟦️.ts", [
        ("cancelled: boolean; heldLoggedMs: number; socket: Socket };",
         "cancelled: boolean; heldLoggedMs: number; socket: Socket; forced?: boolean; pingedMs?: number };"),
        ("waits?: Partial<typeof ADMISSION_POLICY_V1.waits> }>): Promise<AdmissionBrokerV1> {",
         "waits?: Partial<typeof ADMISSION_POLICY_V1.waits>; bounds?: Partial<typeof ADMISSION_POLICY_V1.bounds> }>): Promise<AdmissionBrokerV1> {"),
        ("  const now = options.now ?? Date.now, endpoint = options.endpoint ?? brokerEndpointV1(), passes = new Map<string, BrokerPassV1>();\n",
         "  const now = options.now ?? Date.now, endpoint = options.endpoint ?? brokerEndpointV1(), passes = new Map<string, BrokerPassV1>(), bounds = { ...ADMISSION_POLICY_V1.bounds, ...options.bounds };\n"),
        ("    const admitted = admitWaitersV1(state, ordered);\n    let held = false;\n",
         "    const admitted = admitWaitersV1(state, ordered);\n    const forced = running.size === 0 && !admitted.length ? ordered.findIndex((entry) => entry.forced) : -1;\n    if (forced >= 0) admitted.push(forced);\n    let held = false;\n"),
        ("  const waitsTimer = options.onWaits || options.log || options.growth ? setInterval(() => { waitsTick(); rescueStalled(); }, options.waits?.publishMilliseconds ?? ADMISSION_POLICY_V1.waits.publishMilliseconds) : undefined;\n",
         "  const watchdog = (): void => {\n"
         "    const at = now(), say = (name: keyof typeof schema.definitions.Messages.const.en.waits, values: Readonly<Record<string, string | number>>): void => { try { options.log?.(`[admission] ${formatWaitV1(\"en\", name, values)} / ${formatWaitV1(\"de\", name, values)}`); } catch { return; } };\n"
         "    const nameOf = (entry: Entry): string => entry.request.crate === buildScriptCrateV1 ? entry.key : entry.request.crate;\n"
         "    let changed = false;\n"
         "    for (const entry of [...waiting]) {\n"
         "      const waitedMs = at - entry.requestMs;\n"
         "      if (waitedMs >= bounds.queueMilliseconds) {\n"
         "        waiting.splice(waiting.indexOf(entry), 1);\n"
         "        giveBackGrowth(entry);\n"
         "        lastActivityMs = at;\n"
         "        changed = true;\n"
         "        say(\"expired\", { unit: nameOf(entry), seconds: Math.round(waitedMs / 1000), minutes: Math.ceil(bounds.queueMilliseconds / 60000) });\n"
         "        const wire = admitDefinition(\"RefusalV1\", { v: 1, op: \"refused\", reason: \"parent-memory\", predictedBytes: entry.predictedBytes, allowanceBytes: parentAllowance ?? 0 });\n"
         "        if (!entry.socket.destroyed) entry.socket.end(JSON.stringify(wire) + \"\\n\");\n"
         "        continue;\n"
         "      }\n"
         "      if (at - (entry.pingedMs ?? entry.requestMs) >= bounds.heartbeatMilliseconds) { entry.pingedMs = at; verdict(entry.socket, false); }\n"
         "    }\n"
         "    const first = admissionOrderV1(waiting)[0];\n"
         "    if (!first || running.size > 0 || first.forced || rescuing.has(first) || at - first.requestMs < bounds.stallMilliseconds) { if (changed) schedule(); return; }\n"
         "    first.forced = true;\n"
         "    say(\"stalled\", { unit: nameOf(first), seconds: Math.round((at - first.requestMs) / 1000), predicted: Math.ceil(first.predictedBytes / 1048576), budget: Math.round(budget() / 1048576) });\n"
         "    schedule();\n"
         "  };\n"
         "  const waitsTimer = setInterval(() => { waitsTick(); rescueStalled(); watchdog(); }, options.waits?.publishMilliseconds ?? ADMISSION_POLICY_V1.waits.publishMilliseconds);\n"),
        ("  waitsTimer?.unref?.();\n", "  waitsTimer.unref?.();\n"),
        ("if (waitsTimer) clearInterval(waitsTimer);", "clearInterval(waitsTimer);"),
        # growth deadline and heartbeat
        ("    const ticker = setInterval(() => { const facts = safely(port.local); say(\"waiting\", { ...base, extra, available: facts?.availableMegabytes ?? \"?\", reserve: facts ? facts.floorMegabytes + facts.interactiveReserveMegabytes : \"?\", seconds: seconds() }); }, policy.progressMilliseconds);\n",
         "    const ticker = setInterval(() => { const facts = safely(port.local); say(\"waiting\", { ...base, extra, available: facts?.availableMegabytes ?? \"?\", reserve: facts ? facts.floorMegabytes + facts.interactiveReserveMegabytes : \"?\", seconds: seconds() }); queued(); }, policy.progressMilliseconds);\n"),
        ("    growing.add(growth);\n    try {\n      const answer = await port.acquire({ unit, extraMegabytes: extra, signal, waitMilliseconds, write: (line) => { try { options.log?.(line); } catch { return; } } });\n",
         "    growing.add(growth);\n    const inner = new AbortController(), relay = (): void => inner.abort();\n    signal.addEventListener(\"abort\", relay, { once: true });\n    let guard: ReturnType<typeof setTimeout> | undefined;\n    try {\n      const acquired = port.acquire({ unit, extraMegabytes: extra, signal: inner.signal, waitMilliseconds, write: (line) => { try { options.log?.(line); } catch { return; } } });\n      const late = new Promise<Readonly<{ granted: false; reason: \"timeout\" }>>((accept) => { guard = setTimeout(() => { inner.abort(); accept({ granted: false, reason: \"timeout\" }); }, waitMilliseconds + bounds.growthGraceMilliseconds); guard.unref?.(); });\n      void acquired.then((outcome) => { if (outcome.granted && inner.signal.aborted && !signal.aborted) outcome.release(); }, () => undefined);\n      const answer = await Promise.race([acquired, late]);\n"),
        ("    } finally { clearInterval(ticker); growing.delete(growth); }\n",
         "    } finally { clearInterval(ticker); clearTimeout(guard); signal.removeEventListener(\"abort\", relay); growing.delete(growth); }\n"),
        # predict deadline
        ("          const { key, history, lines, predictedBytes: predicted, cancelled, cold } = await predict(request, pass);\n",
         "          const { key, history, lines, predictedBytes: predicted, cancelled, cold } = await new Promise<Awaited<ReturnType<typeof predict>>>((accept, reject) => {\n"
         "            const timer = setTimeout(() => { try { options.log?.(`[admission] ${formatWaitV1(\"en\", \"late\", { unit: request.crate, seconds: Math.round(bounds.predictMilliseconds / 1000) })} / ${formatWaitV1(\"de\", \"late\", { unit: request.crate, seconds: Math.round(bounds.predictMilliseconds / 1000) })}`); } catch { void 0; } reject(Error(`Admission estimate of ${request.crate} exceeded ${bounds.predictMilliseconds} ms`)); }, bounds.predictMilliseconds);\n"
         "            timer.unref?.();\n"
         "            predict(request, pass).then(accept, reject).finally(() => clearTimeout(timer));\n"
         "          });\n"),
        # probe: a fleet-only pass has nothing to wait for
        ("          let wait = true;\n          if(pass.placement!==\"fleet-only\"){",
         "          let wait = false;\n          if(pass.placement!==\"fleet-only\"){"),
    ])

if which == "rust":
    edit(ADM + "🦀️.rs", [
        ("""struct Broker {
    writer: Mutex<Box<dyn Write + Send>>,
    reader: Mutex<Box<dyn BufRead + Send>>,
}
""", """struct Broker {
    writer: Mutex<Box<dyn Write + Send>>,
    lines: Mutex<std::sync::mpsc::Receiver<std::io::Result<String>>>,
    #[cfg(unix)]
    closer: std::os::unix::net::UnixStream,
}

/// 🔇️ How long the broker may stay silent (no verdict and no queue heartbeat) before the request is dropped and sent again; equals `Policy.bounds.silenceMilliseconds` of the admission schema.
const ADMISSION_SILENCE: Duration = Duration::from_millis(180_000);
/// 🔁️ How many times one admission request is sent in total (a broker that restarted answers the second one); equals `Policy.bounds.attempts`.
const ADMISSION_ATTEMPTS: u32 = 5;
"""),
        ("""        let reader = stream.try_clone()?;
        Ok(Broker { writer: Mutex::new(Box::new(stream)), reader: Mutex::new(Box::new(BufReader::new(reader))) })
    }
""", """        let reader = stream.try_clone()?;
        #[cfg(unix)]
        let closer = stream.try_clone()?;
        let (sender, lines) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(reader);
            loop {
                let mut line = String::new();
                let item = match reader.read_line(&mut line) {
                    Ok(0) => Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "broker closed")),
                    Ok(_) => Ok(line),
                    Err(error) => Err(error),
                };
                let last = item.is_err();
                if sender.send(item).is_err() || last { break; }
            }
        });
        Ok(Broker { writer: Mutex::new(Box::new(stream)), lines: Mutex::new(lines), #[cfg(unix)] closer })
    }
"""),
        ("""    /// 📥️ Waits for the next verdict; `true` once admitted.
    fn verdict(&self) -> std::io::Result<bool> {
        let mut line = String::new();
        let mut reader = self.reader.lock().map_err(|_| std::io::Error::other("broker reader poisoned"))?;
        if reader.read_line(&mut line)? == 0 { return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "broker closed")); }
        decode_verdict(line.trim_end())
    }
}
""", """    /// 📥️ Waits for the next verdict; `true` once admitted. A broker that sends no line for `silence` is reported as `TimedOut`.
    fn verdict(&self, silence: Duration) -> std::io::Result<bool> {
        let lines = self.lines.lock().map_err(|_| std::io::Error::other("broker reader poisoned"))?;
        match lines.recv_timeout(silence) {
            Ok(line) => decode_verdict(line?.trim_end()),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, format!("broker silent for {silence:?}"))),
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "broker closed")),
        }
    }
}

impl Drop for Broker {
    /// 🔌️ Closes the socket so the broker forgets this request and the reader thread ends.
    fn drop(&mut self) {
        #[cfg(unix)]
        let _ = self.closer.shutdown(std::net::Shutdown::Both);
    }
}
"""),
        ("""/// ⏳️ Waits for the broker's admission while KEEPING the jobserver token Cargo gave this job.""",
         """/// 🔁️ Whether an admission fault means the broker is gone, restarting or silent (the request is sent again) rather than answering with a refusal.
fn transient_admission(error: &std::io::Error) -> bool {
    use std::io::ErrorKind::*;
    matches!(error.kind(), TimedOut | UnexpectedEof | ConnectionReset | ConnectionRefused | ConnectionAborted | BrokenPipe | NotFound)
}

/// 🔁️ Runs one admission attempt after another while the broker is unreachable or silent, at most `attempts` times; a refusal or a malformed line ends at once.
/// @see ./🧪️tests/⌛️deadlines/🟦️.ts
fn admit_with_retries<T>(mut attempt: impl FnMut() -> std::io::Result<T>, attempts: u32, pause: impl Fn(u32) -> Duration) -> std::io::Result<T> {
    let mut number = 1;
    loop {
        match attempt() {
            Err(error) if number < attempts && transient_admission(&error) => {
                std::thread::sleep(pause(number));
                number += 1;
            }
            other => return other,
        }
    }
}

/// ⏳️ Waits for the broker's admission while KEEPING the jobserver token Cargo gave this job."""),
        ("""fn await_admission(mut verdict: impl FnMut() -> Result<bool, String>) -> Result<(), String> {
    while !verdict()? {}
    Ok(())
}
""", """fn await_admission(mut verdict: impl FnMut() -> std::io::Result<bool>) -> std::io::Result<()> {
    while !verdict()? {}
    Ok(())
}
"""),
        ("""    let broker = Arc::new(Broker::connect(endpoint).map_err(|error| format!("build broker unreachable at {endpoint}: {error}"))?);
    broker.send(&admit_request(unit, pass, std::process::id())).map_err(|error| format!("broker admission request failed: {error}"))?;
    await_admission(|| broker.verdict().map_err(|error| format!("{} ({}): compiler broker verdict failed: {error}", unit.crate_name, unit.target)))?;
""", """    let broker = admit_with_retries(|| {
        let broker = Arc::new(Broker::connect(endpoint)?);
        broker.send(&admit_request(unit, pass, std::process::id()))?;
        await_admission(|| broker.verdict(ADMISSION_SILENCE))?;
        Ok(broker)
    }, ADMISSION_ATTEMPTS, |attempt| Duration::from_secs((1u64 << attempt).min(15))).map_err(|error| format!("{} ({}): compiler broker admission failed at {endpoint} after up to {ADMISSION_ATTEMPTS} attempts: {error}", unit.crate_name, unit.target))?;
"""),
        ("""        for refusals in [0usize, 1, 7] {
            let mut calls = 0;
            let result = await_admission(|| { calls += 1; Ok(calls > refusals) });
            assert_eq!(result, Ok(()));
            assert_eq!(calls, refusals + 1);
        }
        assert_eq!(await_admission(|| Err("broker closed".to_string())), Err("broker closed".to_string()));
    }
""", """        for refusals in [0usize, 1, 7] {
            let mut calls = 0;
            let result = await_admission(|| { calls += 1; Ok(calls > refusals) });
            assert!(result.is_ok());
            assert_eq!(calls, refusals + 1);
        }
        assert_eq!(await_admission(|| Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "broker closed"))).unwrap_err().to_string(), "broker closed");
    }

    /// 🔇️ A broker that never answers is reported after the silence bound, and the request is sent again until it is answered or the attempts are used up; a refusal is never repeated.
    #[cfg(unix)]
    #[test]
    fn silent_broker_is_dropped_and_asked_again() {
        let directory = env::temp_dir().join(format!("semio-silent-broker-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("broker.sock");
        let _ = std::fs::remove_file(&path);
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        let held = std::thread::spawn(move || {
            let mut kept = Vec::new();
            for index in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                let mut line = String::new();
                BufReader::new(stream.try_clone().unwrap()).read_line(&mut line).unwrap();
                if index == 1 { stream.write_all(b"{\\"v\\":1,\\"op\\":\\"verdict\\",\\"admitted\\":false}\\n{\\"v\\":1,\\"op\\":\\"verdict\\",\\"admitted\\":true}\\n").unwrap(); }
                kept.push(stream);
            }
            kept
        });
        let endpoint = path.to_str().unwrap().to_string();
        let mut attempts = 0;
        let result = admit_with_retries(|| {
            attempts += 1;
            let broker = Broker::connect(&endpoint)?;
            broker.send("{\\"v\\":1,\\"op\\":\\"admit\\"}\\n")?;
            await_admission(|| broker.verdict(Duration::from_millis(300)))?;
            Ok(())
        }, 5, |_| Duration::ZERO);
        assert!(result.is_ok());
        assert_eq!(attempts, 2);
        drop(held.join().unwrap());
        let mut calls = 0;
        let refused = admit_with_retries(|| { calls += 1; Err::<(), _>(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "refused")) }, 5, |_| Duration::ZERO);
        assert_eq!(refused.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
        assert_eq!(calls, 1);
        let mut gone = 0;
        let exhausted = admit_with_retries(|| { gone += 1; Err::<(), _>(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "restarting")) }, 3, |_| Duration::ZERO);
        assert_eq!(exhausted.unwrap_err().kind(), std::io::ErrorKind::ConnectionRefused);
        assert_eq!(gone, 3);
        let _ = std::fs::remove_dir_all(&directory);
    }
"""),
    ])

if which == "wrapper":
    edit(UC + "🧬️schema/🔣️.json", [
        ('          "timeoutMilliseconds": 5000\n        },\n        "evictions": {',
         '          "timeoutMilliseconds": 5000,\n          "headroomPauseMilliseconds": 2000,\n          "headroomMaximumWaitMilliseconds": 60000\n        },\n        "evictions": {'),
    ])
    edit(UC + "🛰️broker/🟦️.ts", [
        ("/** 🔭️ Asks the build's admission broker whether a unit with these facts would queue behind its memory budget right now,",
         "/** ⏳️ Waits for the broker's headroom BEFORE the compile slot is taken: a wrapper that held the single slot while its scheduler waited for admission froze every other unit behind it (one stuck admission, 28 queued units). Polls the broker's `probe` (never joins its queue) every `headroomPauseMilliseconds` until it would admit the unit or `headroomMaximumWaitMilliseconds` passed; `\"none\"` when no broker answers, `\"timeout\"` when the bound passed (the caller then goes on, and the broker's own bounds end the wait).\n * @see ./../🧪️tests/⌛️headroom-first/🟦️.ts */\n"
         "export async function awaitBrokerHeadroomV1(environment:Environment,facts:AdmitFactsV1,options:Readonly<{maximumWaitMilliseconds?:number;pauseMilliseconds?:number}>={}):Promise<\"admit\"|\"timeout\"|\"none\">{\n"
         " const broker=compilerBrokerOfV1(environment);if(!broker)return\"none\";\n"
         " const started=Date.now(),limit=options.maximumWaitMilliseconds??policy.headroomMaximumWaitMilliseconds,pause=options.pauseMilliseconds??policy.headroomPauseMilliseconds;\n"
         " for(;;){\n"
         "  const row=await exchange(broker.endpoint,{v:1,op:\"probe\",pass:broker.pass,pid:process.pid,...facts},\"headroom\");\n"
         "  if(typeof row?.wait!==\"boolean\")return\"none\";\n"
         "  if(!row.wait)return\"admit\";\n"
         "  const waited=Date.now()-started;if(waited>=limit)return\"timeout\";\n"
         "  await new Promise(done=>setTimeout(done,Math.min(pause,limit-waited)));\n"
         " }\n"
         "}\n\n"
         "/** 🔭️ Asks the build's admission broker whether a unit with these facts would queue behind its memory budget right now,"),
    ])
    edit(UC + "📜️script.ts", [
        ("import {brokerOfV1,brokerWouldWaitV1,compilerBrokerOfV1,",
         "import {awaitBrokerHeadroomV1,brokerOfV1,brokerWouldWaitV1,compilerBrokerOfV1,"),
        ("  let slot=held;held=undefined;\n  if(slot===undefined&&settings.slots>0&&unit.source.endsWith(\".rs\")){try{slot=",
         "  let slot=held;held=undefined;\n  if(slot===undefined&&settings.slots>0&&unit.source.endsWith(\".rs\")){const facts=admitFactsV1(args,environment,cwd);if(facts&&await awaitBrokerHeadroomV1(environment,facts)===\"timeout\")event(\"slot-wait\",null,0,0,\"headroom: the broker did not admit the unit in time, taking the slot\");}\n  if(slot===undefined&&settings.slots>0&&unit.source.endsWith(\".rs\")){try{slot="),
    ])

if which == "growthpolicy":
    edit(ADM + "🟦️.ts", [
        ("bounds?: Partial<typeof ADMISSION_POLICY_V1.bounds> }>)", "bounds?: Partial<typeof ADMISSION_POLICY_V1.bounds>; growthPolicy?: Partial<typeof ADMISSION_POLICY_V1.growth> }>)"),
        ("bounds = { ...ADMISSION_POLICY_V1.bounds, ...options.bounds };\n", "bounds = { ...ADMISSION_POLICY_V1.bounds, ...options.bounds }, growthPolicy = { ...ADMISSION_POLICY_V1.growth, ...options.growthPolicy };\n"),
        ("waitMilliseconds: number = ADMISSION_POLICY_V1.growth.waitMilliseconds): Promise<GrowthOutcome> => {\n    const port = options.growth!, policy = ADMISSION_POLICY_V1.growth,",
         "waitMilliseconds: number = growthPolicy.waitMilliseconds): Promise<GrowthOutcome> => {\n    const port = options.growth!, policy = growthPolicy,"),
        ("() => verdict(socket, false), ADMISSION_POLICY_V1.growth.overheadWaitMilliseconds);", "() => verdict(socket, false), growthPolicy.overheadWaitMilliseconds);"),
    ])

if which == "fleetonly":
    edit(ADM + "🧬️schema/🔣️.json", [
        ('        "wait": {\n          "type": "boolean"\n        }\n      }\n    },\n    "RecallRequestV1": {',
         '        "wait": {\n          "type": "boolean"\n        },\n        "fleetOnly": {\n          "description": "The pass is fleet-only: local compilation is refused, so a wrapper has no headroom to wait for.",\n          "const": true\n        }\n      }\n    },\n    "RecallRequestV1": {'),
    ])
    edit(ADM + "🟦️.ts", [
        ("          let wait = false;\n          if(pass.placement!==\"fleet-only\"){", "          let wait = true;\n          if(pass.placement!==\"fleet-only\"){"),
        ("socket.end(`{\"v\":1,\"op\":\"headroom\",\"wait\":${wait}}\\n`);", "socket.end(`{\"v\":1,\"op\":\"headroom\",\"wait\":${wait}${pass.placement===\"fleet-only\"?\",\\\"fleetOnly\\\":true\":\"\"}}\\n`);"),
    ])
    edit(UC + "🛰️broker/🟦️.ts", [
        ("  if(typeof row?.wait!==\"boolean\")return\"none\";\n  if(!row.wait)return\"admit\";", "  if(typeof row?.wait!==\"boolean\"||row.fleetOnly===true)return\"none\";\n  if(!row.wait)return\"admit\";"),
    ])
    edit(ADM + "🧪️tests/⌛️deadlines/🟦️.ts", [
        ('test("a fleet-only pass has nothing to wait for: the headroom probe answers admit so that the wrapper never waits outside the slot for a refusal"', 'test("a fleet-only pass says so in its headroom probe, so that the wrapper never waits outside the slot for a refusal"'),
        ('toEqual({v:1,op:"headroom",wait:false})', 'toEqual({v:1,op:"headroom",wait:true,fleetOnly:true})'),
    ])
