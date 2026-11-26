import os, signal, subprocess, sys, time
from pathlib import Path
ticket=Path(__file__).resolve().parent
gen=ticket/'🗑️generated'
root=ticket.parents[6]
manifest=next(root.glob('✏*/🔌*/🧩*/🗿*/🧊*/🏅*/🔖*/🪆*/✳*/✏*/📦*/🦀*/Cargo.toml')).parent
delay = int(sys.argv[1]) if len(sys.argv) > 1 else 18
with (gen/f'current-fill-live-stacks-{delay}.log').open('w') as log:
 process=subprocess.Popen([gen/'current-3d-source-native/history-tests','--exact','publish_fill_count_from_original_window','--nocapture'],cwd=manifest,stdout=log,stderr=log,env={**os.environ, 'RUST_MIN_STACK':'134217728','LD_PRELOAD':str(gen/'current-native-stack.so')})
 time.sleep(delay)
 for i in range(120):
  if process.poll() is not None:break
  for task in Path(f'/proc/{process.pid}/task').iterdir():
   if (task/'comm').read_text().startswith('publish_fill'):
    os.kill(int(task.name), signal.SIGUSR2)
  time.sleep(.11)
 process.wait()
