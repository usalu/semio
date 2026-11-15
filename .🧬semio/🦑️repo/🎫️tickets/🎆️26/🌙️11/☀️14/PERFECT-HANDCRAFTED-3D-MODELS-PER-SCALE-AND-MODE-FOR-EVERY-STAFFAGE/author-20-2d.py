import json, re, sys

CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/%s/%s/🔣️.json"


class Doc:
    def __init__(self, cat, slug):
        self.path = CAT % (cat, slug)
        self.lines = open(self.path, encoding="utf-8").read().split("\n")

    def scale_range(self, scale):
        i = next(k for k, l in enumerate(self.lines) if l == '  "representations": {')
        a = next(k for k in range(i, len(self.lines)) if self.lines[k] == '   "%s": {' % scale)
        b = next(k for k in range(a + 1, len(self.lines)) if re.match(r'   "\d+": \{|  \}', self.lines[k]))
        return a, b

    def view_range(self, scale, view):
        a, b = self.scale_range(scale)
        v = next(k for k in range(a, b) if self.lines[k] == '     "%s": [' % view)
        e = next(k for k in range(v + 1, b) if self.lines[k] in ("     ],", "     ]"))
        return v, e

    def shape_range(self, scale, view, sid):
        v, e = self.view_range(scale, view)
        for k in range(v + 1, e):
            if self.lines[k] == '       "id": "%s",' % sid:
                s = k - 1
                t = next(j for j in range(k, e + 1) if self.lines[j] in ("      },", "      }"))
                return s, t
        raise KeyError((scale, view, sid))

    def delete(self, scale, view, sid):
        s, t = self.shape_range(scale, view, sid)
        last = self.lines[t] == "      }"
        del self.lines[s : t + 1]
        if last:
            self.lines[s - 1] = self.lines[s - 1].rstrip(",")

    def set(self, scale, view, sid, **fields):
        s, t = self.shape_range(scale, view, sid)
        for key, val in fields.items():
            for k in range(s, t + 1):
                m = re.match(r'(       "%s": )(.*?)(,?)$' % key, self.lines[k])
                if m:
                    self.lines[k] = m.group(1) + json.dumps(val) + m.group(3)
                    break
            else:
                raise KeyError(key)

    def points(self, scale, view, sid, pts):
        s, t = self.shape_range(scale, view, sid)
        a = next(k for k in range(s, t) if self.lines[k].startswith('       "points": ['))
        b = next(k for k in range(a, t + 1) if self.lines[k] in ("       ]", "       ],"))
        tail = "," if self.lines[b].endswith(",") else ""
        self.lines[a : b + 1] = ['       "points": ' + json.dumps(pts) + tail]

    def save(self):
        t = "\n".join(self.lines)
        json.loads(t)
        open(self.path, "w", encoding="utf-8").write(t)
