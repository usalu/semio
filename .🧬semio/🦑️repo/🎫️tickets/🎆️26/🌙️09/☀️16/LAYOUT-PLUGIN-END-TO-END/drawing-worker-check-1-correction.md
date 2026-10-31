# Worker Cancellation Return Compiler Correction

The actual compiler RED is corrected only at the stale cancellation-return arm. It captures the defining turn, preserves None/Blocked as Blocked, and passes full result/work through receive_original_retained_work before returning the public Worker close step. It no longer feeds this arm into the old Result-only close helper. All cases, budgets and grants remain unchanged.

Before SHA256 63d0d263ec9dbd163ed2346d2f1e249344816c49658829135f86846f06201a6a; after SHA256 6c7f6d8b962d6de1a39813b8d47d6125a35d58961d80313c6ce3d839e6c53e09. Other legacy close helpers still infer CPU from receipt items and remain explicit blockers for owning closure.
