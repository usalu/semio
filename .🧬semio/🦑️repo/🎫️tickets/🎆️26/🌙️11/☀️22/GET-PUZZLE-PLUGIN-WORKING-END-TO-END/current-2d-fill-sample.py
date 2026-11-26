from pathlib import Path
import os, subprocess, signal, time
ticket=Path(__file__).resolve().parent
generated=ticket/'🗑️generated'
binary=generated/'current-2d-hist-relink/history-fresh-fill'
log=generated/'current-2d-fill-stacks.log'
with log.open('w') as out:
    process=subprocess.Popen([binary,'fill_run_start_complete_finalize_is_one_undo_entry','--nocapture'],stdout=out,stderr=out,env={**os.environ,'LD_PRELOAD':str(generated/'current-native-stack.so'),'RUST_MIN_STACK':'134217728'})
    for delay in (5,10,20,40,50):
        time.sleep(delay)
        if process.poll() is not None:break
        for task in Path(f'/proc/{process.pid}/task').iterdir():
            if task.name!=str(process.pid) and (task/'comm').read_text().startswith('mirror::'):
                os.kill(int(task.name),signal.SIGUSR2)
    process.wait()
