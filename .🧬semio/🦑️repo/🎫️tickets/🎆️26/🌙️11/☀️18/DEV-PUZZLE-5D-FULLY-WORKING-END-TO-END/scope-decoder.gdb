set pagination off
set breakpoint pending off
set print thread-events off
break '<semio_framework_job_context::component::original_job_decoder::OriginalJobDecodeOwner>::close_on_source'
run
set $base = $pc - 0xd233bd0
ignore 1 100
continue
tbreak *($base+0xd233cd8)
tbreak *($base+0xd233dc7)
tbreak *($base+0xd234042)
tbreak *($base+0xd23411f)
continue
print/x $pc - $base
bt 5
x/5gd $rsp+0x5558
quit
