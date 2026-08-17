#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <zlib.h>

static unsigned char *read_file(const char *path, size_t *length) {
    FILE *file = fopen(path, "rb");
    fseek(file, 0, SEEK_END);
    *length = (size_t)ftell(file);
    rewind(file);
    unsigned char *bytes = malloc(*length);
    fread(bytes, 1, *length, file);
    fclose(file);
    return bytes;
}

static void trace_blocks(const char *name, const unsigned char *bytes, size_t length) {
    unsigned char output[32768];
    z_stream stream = {0};
    inflateInit(&stream);
    stream.next_in = (unsigned char *)bytes;
    stream.avail_in = (uInt)length;
    stream.next_out = output;
    stream.avail_out = sizeof(output);
    printf("%s blocks", name);
    while (1) {
        int result = inflate(&stream, Z_BLOCK);
        printf(" [in=%lu bit=%lu out=%lu last=%u boundary=%u]", stream.total_in,
               stream.total_in * 8 - (stream.data_type & 7), stream.total_out,
               (stream.data_type & 64) != 0, (stream.data_type & 128) != 0);
        if (result == Z_STREAM_END) break;
        if (result != Z_OK || stream.avail_out == 0) {
            printf(" error=%d", result);
            break;
        }
    }
    printf("\n");
    inflateEnd(&stream);
}

static size_t compress_with_flush(const unsigned char *decoded, size_t decoded_length,
                                  unsigned char *candidate, size_t candidate_capacity,
                                  int flush) {
    z_stream stream = {0};
    deflateInit2(&stream, 6, Z_DEFLATED, 12, 5, Z_DEFAULT_STRATEGY);
    stream.next_in = (unsigned char *)decoded;
    stream.avail_in = (uInt)decoded_length;
    stream.next_out = candidate;
    stream.avail_out = (uInt)candidate_capacity;
    if (deflate(&stream, flush) != Z_OK) return 0;
    if (deflate(&stream, Z_FINISH) != Z_STREAM_END) return 0;
    size_t length = stream.total_out;
    deflateEnd(&stream);
    return length;
}

static void compare(const char *name, const unsigned char *expected, size_t expected_length,
                    const unsigned char *actual, size_t actual_length) {
    size_t first_diff = 0;
    while (first_diff < expected_length && first_diff < actual_length &&
           expected[first_diff] == actual[first_diff]) ++first_diff;
    printf("%s length=%zu first_diff=%zu expected=%02x actual=%02x\n", name, actual_length,
           first_diff, expected[first_diff], actual[first_diff]);
    trace_blocks(name, actual, actual_length);
}

int main(void) {
    size_t fixture_length;
    unsigned char *fixture = read_file("temp/📄️bachelor-thesis.pdf", &fixture_length);
    const char marker[] = "/Length 3362\n/Filter /FlateDecode\n>>\nstream\n";
    unsigned char *start = NULL;
    for (size_t index = 0; index + sizeof(marker) - 1 <= fixture_length; ++index) {
        if (memcmp(fixture + index, marker, sizeof(marker) - 1) == 0) {
            start = fixture + index + sizeof(marker) - 1;
            break;
        }
    }
    if (!start) return 2;
    unsigned char decoded[32768];
    uLongf decoded_length = sizeof(decoded);
    if (uncompress(decoded, &decoded_length, start, 3362) != Z_OK) return 3;
    uLongf candidate_capacity = compressBound(decoded_length);
    unsigned char *candidate = malloc(candidate_capacity);
    z_stream stream = {0};
    deflateInit2(&stream, 6, Z_DEFLATED, 12, 5, Z_DEFAULT_STRATEGY);
    stream.next_in = decoded;
    stream.avail_in = (uInt)decoded_length;
    stream.next_out = candidate;
    stream.avail_out = (uInt)candidate_capacity;
    if (deflate(&stream, Z_FINISH) != Z_STREAM_END) return 4;
    size_t candidate_length = stream.total_out;
    deflateEnd(&stream);
    size_t first_diff = 0;
    while (first_diff < 3362 && first_diff < candidate_length &&
           start[first_diff] == candidate[first_diff]) ++first_diff;
    printf("system-zlib=%s candidate_length=%zu first_diff=%zu expected=%02x actual=%02x\n",
           zlibVersion(), candidate_length, first_diff, start[first_diff], candidate[first_diff]);
    trace_blocks("fixture", start, 3362);
    trace_blocks("system-w12-l6-m5-default", candidate, candidate_length);
    candidate_length = compress_with_flush(decoded, decoded_length, candidate,
                                           candidate_capacity, Z_PARTIAL_FLUSH);
    compare("system-partial-then-finish", start, 3362, candidate, candidate_length);
    candidate_length = compress_with_flush(decoded, decoded_length, candidate,
                                           candidate_capacity, Z_SYNC_FLUSH);
    compare("system-sync-then-finish", start, 3362, candidate, candidate_length);
    candidate_length = compress_with_flush(decoded, decoded_length, candidate,
                                           candidate_capacity, Z_FULL_FLUSH);
    compare("system-full-then-finish", start, 3362, candidate, candidate_length);
    free(candidate);
    free(fixture);
    return 0;
}
