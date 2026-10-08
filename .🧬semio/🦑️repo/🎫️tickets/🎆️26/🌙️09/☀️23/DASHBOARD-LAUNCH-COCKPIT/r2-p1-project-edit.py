p='📋️project.json'
s=open(p,encoding='utf8',newline='').read()
nl='\r\n' if '\r\n' in s else '\n'
assert nl=='\n'
old_http='''            "id": "os-mcp-http",
            "verb": "dev",
            "command": [
              "bun",
              "nx",
              "run",
              "workspace:dev",
              "--",
              "mcp",
              "http",
              "os"
            ],
            "continuous": true
          },'''
new_http='''            "id": "os-mcp-http",
            "verb": "dev",
            "command": [
              "bun",
              "nx",
              "run",
              "@semio-tech/framework-os-mcp-rs:dev",
              "--",
              "http",
              "--port",
              "6300"
            ],
            "continuous": true,
            "ready": {
              "port": 6300
            }
          },'''
assert s.count(old_http)==1
s=s.replace(old_http,new_http)
old_sb='''"command": ["bun", "nx", "run", "workspace:dev", "--", "storybook-static"],'''
assert s.count(old_sb)==1
s=s.replace(old_sb,'''"command": ["bun", "./📜️script.ts", "dev", "storybook-static"],''')
old_dev='''    "dev": {
      "executor": "nx:run-commands",
      "cache": false,
      "continuous": true,
      "options": {
        "command": "bun ./📜️script.ts dev",
        "forwardAllArgs": true
      }
    },
    "dev-storybook": {'''
assert s.count(old_dev)==1
s=s.replace(old_dev,'    "dev-storybook": {')
old_eng='''    "dev-mcp-engine": {
      "executor": "nx:run-commands",
      "cache": false,
      "continuous": true,
      "options": {
        "command": "bun ./📜️script.ts dev mcp engine",
        "forwardAllArgs": true
      }
    },
'''
assert s.count(old_eng)==1
s=s.replace(old_eng,'')
f=open(p,'r+',encoding='utf8',newline=''); f.seek(0); f.write(s); f.truncate(); f.close()
