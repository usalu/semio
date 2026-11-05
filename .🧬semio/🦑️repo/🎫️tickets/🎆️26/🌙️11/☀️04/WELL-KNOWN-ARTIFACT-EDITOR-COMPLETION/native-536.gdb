set pagination off
set confirm off
set breakpoint pending on
set print thread-events off
break malloc if $rdi == 536
commands
silent
printf "[DEBUG] malloc536\n"
bt 22
continue
end
run --exact identity::actual_native_window_app_opens_and_closes_with_its_original_registered_sources --nocapture
