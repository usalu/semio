import os,re,sys
root="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/"
def edit(path,fn):
    p=root+path;t=open(p,encoding="utf8").read();n=fn(t)
    assert n!=t,path
    tmp=p+".m7.tmp";open(tmp,"w",encoding="utf8").write(n);os.replace(tmp,p)
PLAT={"host.cpu.load":["darwin","linux"],"host.memory.compressed":["darwin"],"host.swap.used":["darwin","linux"],"host.swap.rate":["darwin","linux"],"host.files.open":["darwin","linux"],"host.images.attached":["darwin"],"warden.headroom":["darwin","linux"],"process.repo.cpu":["darwin","linux"]}
def cat(t):
    for id,p in PLAT.items():
        pat=re.compile(r'(      "id": "%s",\n(?:      .*\n)*?      "scope": "[a-z]+",\n)'%re.escape(id))
        assert pat.search(t),id
        t=pat.sub(lambda m:m.group(1)+'      "platforms": [%s],\n'%", ".join('"%s"'%x for x in p),t,count=1)
    t=t.replace('''        "en": "Sampler CPU share",
        "de": "CPU-Anteil des Samplers"''','''        "en": "Collector CPU share (its own steps)",
        "de": "CPU-Anteil des Sammlers (seine eigenen Schritte)"''')
    t=t.replace('''        "en": "Sampler memory",
        "de": "Speicher des Samplers"''','''        "en": "Collector retained memory",
        "de": "Vom Sammler gehaltener Speicher"''')
    new='''    },
    {
      "id": "process.ready.cpu",
      "kind": "gauge",
      "unit": "percent",
      "scope": "origin",
      "source": "self",
      "label": {
        "en": "Readiness process CPU share (whole process)",
        "de": "CPU-Anteil des Bereitschaftsprozesses (ganzer Prozess)"
      }
    },
    {
      "id": "process.ready.rss",
      "kind": "gauge",
      "unit": "bytes",
      "scope": "origin",
      "source": "self",
      "label": {
        "en": "Readiness process resident memory (whole process)",
        "de": "Residenter Speicher des Bereitschaftsprozesses (ganzer Prozess)"
      }
    }
  ]
}
'''
    assert t.rstrip().endswith('    }\n  ]\n}')
    t=t.rstrip()[:-len('    }\n  ]\n}')]+new
    return t
edit("📇️catalogue/🔣️.json",cat)
def schema(t):
    return t.replace('''        "scope": {"enum": ["origin", "fleet"]},
        "source": {"type": "string", "minLength": 1},''','''        "scope": {"enum": ["origin", "fleet"]},
        "platforms": {"description": "The operating systems that can measure this metric (`darwin`, `linux`, `win32`); absent means all. On a machine of another platform the metric is not a gap but `n/a`: the platform has no cheap source for it.", "type": "array", "minItems": 1, "uniqueItems": true, "items": {"enum": ["darwin", "linux", "win32"]}},
        "source": {"type": "string", "minLength": 1},''',1)
edit("🧬️schema/🔣️.json",schema)
