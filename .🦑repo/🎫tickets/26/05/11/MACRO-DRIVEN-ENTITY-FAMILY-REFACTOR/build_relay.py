old = "{ store { wip { theKit { kit { designs { edges { node { id pieces { edges { node { id position { center { u v } } } } } } } } } } } } } }"
marker = "{ store { wip {"
body = old[len(marker) :]
pre = "{ session { stores { edges { node { wip {"
new = pre + body + "} } }"
print(new)
print("opens", new.count("{"), "closes", new.count("}"))
d = 0
for c in new:
    d += 1 if c == "{" else (-1 if c == "}" else 0)
print("depth", d)
