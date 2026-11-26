#define _GNU_SOURCE
#include <signal.h>
#include <execinfo.h>
#include <unistd.h>
#include <ucontext.h>
static void report(int signal, siginfo_t *info, void *context) {
    ((ucontext_t *)context)->uc_mcontext.gregs[REG_RIP]--;
    void *frames[96];
    int count = backtrace(frames, 96);
    backtrace_symbols_fd(frames, count, STDERR_FILENO);
    _exit(128 + signal);
}
__attribute__((constructor)) static void attach(void) {
    struct sigaction action = {.sa_sigaction = report, .sa_flags = SA_SIGINFO | SA_RESTART};
    sigemptyset(&action.sa_mask);
    sigaction(SIGTRAP, &action, 0);
}
