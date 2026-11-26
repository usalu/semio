import os
p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🛰️collector/🧬️schema/🔣️.json"
t=open(p,encoding="utf8").read();o=t
t=t.replace('''        "repoCpuMaximumGapMilliseconds": 60000
''','''        "repoCpuMaximumGapMilliseconds": 60000,
        "openRetryMilliseconds": 30000,
        "appleDoublePrefix": "._"
''',1)
t=t.replace('''            "metrics.self.write.duration"
          ],
          "label": {
            "en": "This collector measuring itself",
            "de": "Dieser Sammler misst sich selbst"''','''            "metrics.self.write.duration",
            "process.ready.cpu",
            "process.ready.rss"
          ],
          "label": {
            "en": "This collector measuring its own steps (CPU time inside its ticks, bytes it retains) and, separately, the whole readiness process that hosts it",
            "de": "Dieser Sammler misst seine eigenen Schritte (CPU-Zeit in seinen Ticks, von ihm gehaltene Bytes) und getrennt den ganzen Bereitschaftsprozess, der ihn beherbergt"''',1)
t=t.replace('''          "failed": "The collector {name} failed: {reason}",
''','''          "failed": "The collector {name} failed: {reason}",
          "opening": "Opening the rings in {directory} failed, trying again later: {reason}",
''',1)
t=t.replace('''          "failed": "Der Sammler {name} ist gescheitert: {reason}",
''','''          "failed": "Der Sammler {name} ist gescheitert: {reason}",
          "opening": "Das Öffnen der Ringe in {directory} ist gescheitert, später erneut: {reason}",
''',1)
assert t.count("openRetry")==1 and t.count('"opening"')==2 and "process.ready.cpu" in t
tmp=p+".m7.tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,p)
