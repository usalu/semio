#include <execinfo.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

static _Thread_local int active;
static _Thread_local unsigned observed;

static void note_extent(const char *label, size_t length) {
    if (length != 392 || active || observed >= 512) return;

    active = 1;
    ++observed;
    void *frames[28];
    int count = backtrace(frames, 28);
    (void)write(STDERR_FILENO, label, 29);
    backtrace_symbols_fd(frames, count, STDERR_FILENO);
    active = 0;
}

static void *observed_malloc(size_t length) {
    void *value = malloc(length);
    note_extent("[DEBUG] Native malloc 392   \n", length);
    return value;
}

static void *observed_calloc(size_t count, size_t width) {
    void *value = calloc(count, width);
    if (width && count <= SIZE_MAX / width) {
        note_extent("[DEBUG] Native calloc 392   \n", count * width);
    }
    return value;
}

static void *observed_realloc(void *previous, size_t length) {
    void *value = realloc(previous, length);
    note_extent("[DEBUG] Native realloc 392  \n", length);
    return value;
}

__attribute__((used, section("__DATA,__interpose"))) static const struct {
    const void *replacement;
    const void *original;
} hooks[] = {
    { (const void *)observed_malloc, (const void *)malloc },
    { (const void *)observed_calloc, (const void *)calloc },
    { (const void *)observed_realloc, (const void *)realloc },
};
