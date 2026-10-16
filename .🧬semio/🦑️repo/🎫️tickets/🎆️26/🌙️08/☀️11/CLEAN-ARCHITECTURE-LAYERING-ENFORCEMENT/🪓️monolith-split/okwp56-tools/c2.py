# 🪙 C2: the mint witness. The publication crate gains `StoreMintWitness` and the witness-gated `mint_for_store`; the store engine presents the token only a live `StoreLog` hands out (PLAN-OS-KERNEL-CRATES.md 5.3, decision D1).
import os, sys, hashlib, re
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import em
PUB = em.S + '/🎮️one-item/📬️publication/🦀️.rs'

def sub1(text, old, new):
    assert text.count(old) == 1, (text.count(old), old[:80])
    return text.replace(old, new)

TRAIT = '''/// 🪪️ The capability a store engine presents to mint a live authority. Only a live store log hands an implementor out (`StoreLog::mint_witness`); a type that implements it elsewhere fabricates a publication authority and is a review finding.
pub trait StoreMintWitness {}

'''
FN = '''    /// 🪙️ The cross-crate entry of [`Self::mint`]: the store engine presents the witness a live store log hands out, so the authority is never minted without one.
    pub fn mint_for_store(
        _witness: &dyn StoreMintWitness,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        base_revision: [u8; 32],
        base_applied_edit_count: usize,
        next_sequence_number: i32,
        next_clock: HybridLogicalTimestamp,
        actor: semio_framework_value::SharedUtf8,
        line: Option<String>,
        group_id: Option<String>,
        stamped_edit_id: Option<String>,
    ) -> Self {
        Self::mint(operation, generation, base_revision, base_applied_edit_count, next_sequence_number, next_clock, actor, line, group_id, stamped_edit_id)
    }

'''
def patch_publication(text):
    text = sub1(text, "impl ArtifactStoreOneItemLiveAuthority {\n    /// 🪙️ Mints the live authority", TRAIT + "impl ArtifactStoreOneItemLiveAuthority {\n    /// 🪙️ Mints the live authority")
    text = sub1(text, "    /// 🔗️ The same publication authority rebound under one durable group decision.\n    pub fn rebind_group", FN + "    /// 🔗️ The same publication authority rebound under one durable group decision.\n    pub fn rebind_group")
    return text

TOKEN = '''/// 🪙️ The witness a live store log hands out for minting publication authorities; it has no public constructor.
pub struct StoreMintToken {
    _sealed: (),
}

impl one_item_publication::StoreMintWitness for StoreMintToken {}

impl<P, Mutation> StoreLog<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// 🪙️ The mint witness of this live log: what the engine presents to [`ArtifactStoreOneItemLiveAuthority::mint_for_store`].
    #[inline]
    pub fn mint_witness(&self) -> &'static StoreMintToken {
        &StoreMintToken { _sealed: () }
    }
}

'''
def patch_root(text):
    # token + witness accessor right before the composer struct
    k = text.index("/// 🗃️ The composer of one store")
    text = text[:k] + TOKEN + text[k:]
    text = sub1(text, "let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint(operation, semio_framework_job::Generation(expected_generation)", "let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint_for_store(store_log.mint_witness(), operation, semio_framework_job::Generation(expected_generation)")
    text = sub1(text, "let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(\n", "let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint_for_store(\n            store_log.mint_witness(),\n")
    return text

def apply():
    for path, fn in ((PUB, patch_publication), (em.ROOT, patch_root)):
        old = open(path, encoding='utf8').read()
        new = fn(old)
        again = open(path, encoding='utf8').read()
        assert again == old, ('changed during edit', path)
        open(path, 'w', encoding='utf8').write(new)
        print('patched', path[-60:])
if __name__ == '__main__':
    apply()
