# Root Private Native Cache Retirement

Only obsolete compiled kernel variants in this ticket’s Root private generated Cargo cache are retired. The four newest variants and every variant modified during the previous 30 minutes are retained. No source, input, report, Git state, or other team cache is removed. Before retirement the filesystem had 388 MiB free.

Candidates: 44; retained newest: 5b382efd5c245630, dbfae166bbffd109, 0c2c47d491677eaf, 4c07b06e70750bbd.

Retired 44 obsolete variants: 9b912398a2ba30fb, 179a42e46e4380f2, f3441ac1e657e84b, 19edb454769edcdc, 499ac2d2b13a96b3, fea92c553c98dc63, 492b331879cc8fa9, 1b95ebf81886c648, 724200d35856a972, f43944efb8a88e71, 7a8444125eb43606, 2bd7eb67ccd187eb, c26754032a8a77db, 9abac8e08e0593c7, f08bbda8c0a5a1b0, 364f2612f2051899, 24c3080ae7045880, bb7de78cd5ae1ee6, e5399b3323c7aaef, c81e56d01c4a8c2d, 9fa4b187caf166ed, 7105106ca2c50054, 27bb27d29c308d31, 33d1b1bd01292ce4, 4a7460a7a40f5dbb, 57aa92c4cdc40461, fb764fa7f35abaef, 2afee39751302d35, 0683e57847ba9079, c9732879e10dbb98, 5f8aefe5fbb7f011, 78444655dd712e7f, 9019188af14d1223, efa8a84ffdeec9ed, f4064c333d83bbf1, 532ab16842b6b934, b8f903eec91e86a0, 9f1080d5ff8ae198, e4eb68fe00036a91, 265538c70a7fb549, 23b1ba0733d3508b, b318effc8f4d560d, 920749b7c7c5c072, 98315041d55dca2f.

The same newest-four and 30-minute exclusion was applied to semio crate variants in this same Root-private cache; 1551 additional obsolete variants across 85 owners were retired. Active work uses recently created/current variants, all preserved.
