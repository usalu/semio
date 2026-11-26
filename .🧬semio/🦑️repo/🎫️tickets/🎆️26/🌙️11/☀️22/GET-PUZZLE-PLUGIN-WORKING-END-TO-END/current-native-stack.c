#define _GNU_SOURCE
#include <execinfo.h>
#include <signal.h>
#include <unistd.h>
static void sample(int sig) {
    void *frames[60];
    int count = backtrace(frames, 60);
    backtrace_symbols_fd(frames, count, STDERR_FILENO);
}
__attribute__((constructor)) static void start(void) {
    struct sigaction action = { .sa_handler = sample, .sa_flags = SA_RESTART };
    sigemptyset(&action.sa_mask);
    sigaction(SIGUSR2, &action, 0);
}
