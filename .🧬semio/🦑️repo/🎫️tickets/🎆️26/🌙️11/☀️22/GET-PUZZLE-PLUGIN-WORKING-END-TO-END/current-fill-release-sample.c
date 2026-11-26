#define _GNU_SOURCE
#include <execinfo.h>
#include <malloc.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/prctl.h>
#include <unistd.h>
extern void __libc_free(void *);
static _Thread_local size_t total, next = 512 * 1024;
static _Thread_local int active, samples;
static const char *gate;
__attribute__((constructor)) static void init(void) { gate = getenv("SEMIO_SAMPLE_AFTER"); }
void free(void *p) {
    if (p && !active && samples < 100) {
        active = 1;
        size_t bytes = malloc_usable_size(p);
        total += bytes;
        if (bytes >= 4080 && bytes <= 4104 && total >= next) {
            next = total + 512 * 1024;
            char name[16] = {0};
            prctl(PR_GET_NAME, name);
            if (name[0] == 'm' && gate && access(gate, F_OK) == 0) {
                void *frames[48];
                ++samples;
                int count = backtrace(frames, 48);
                dprintf(2, "[DEBUG] physical release sample %d bytes %zu total %zu\n", samples, bytes, total);
                backtrace_symbols_fd(frames, count, 2);
            }
        }
        active = 0;
    }
    __libc_free(p);
}
