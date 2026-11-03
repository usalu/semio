//! 🗃️ Static NDP table generated from `🔣️.json` by the ticket codegen: the per-Eurocode row crates joined at compile time into one id-sorted slice for binary search. Edit the JSON, not this file.

use super::NdpEntry;

use semio_s_plugin_norm_ndp_en1990 as en1990;
use semio_s_plugin_norm_ndp_en1991 as en1991;
use semio_s_plugin_norm_ndp_en1992 as en1992;
use semio_s_plugin_norm_ndp_en1993 as en1993;
use semio_s_plugin_norm_ndp_en1994 as en1994;
use semio_s_plugin_norm_ndp_en1995 as en1995;
use semio_s_plugin_norm_ndp_en1996 as en1996;
use semio_s_plugin_norm_ndp_en1997 as en1997;
use semio_s_plugin_norm_ndp_en1998 as en1998;
use semio_s_plugin_norm_ndp_en1999 as en1999;

const PARTS: [&[NdpEntry]; 10] = [en1990::ROWS, en1991::ROWS, en1992::ROWS, en1993::ROWS, en1994::ROWS, en1995::ROWS, en1996::ROWS, en1997::ROWS, en1998::ROWS, en1999::ROWS];

const LEN: usize = {
    let mut total = 0;
    let mut part = 0;
    while part < PARTS.len() {
        total += PARTS[part].len();
        part += 1;
    }
    total
};

const fn join() -> [NdpEntry; LEN] {
    let mut out = [PARTS[0][0]; LEN];
    let (mut index, mut part) = (0, 0);
    while part < PARTS.len() {
        let mut row = 0;
        while row < PARTS[part].len() {
            out[index] = PARTS[part][row];
            index += 1;
            row += 1;
        }
        part += 1;
    }
    out
}

/// 📊️ All registered NDPs and table cells, sorted by id.
pub static NDP_TABLE: &[NdpEntry] = &join();
