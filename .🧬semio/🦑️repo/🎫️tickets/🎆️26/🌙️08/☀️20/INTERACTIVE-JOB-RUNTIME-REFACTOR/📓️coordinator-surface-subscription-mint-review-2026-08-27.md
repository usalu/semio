# Surface Subscription Runtime Authority Review

A new bounded source-review question follows the read-commit mint repair. It has not yet been executed as a runtime regression at this checkpoint.

`Subscription` in the retained Surface module is module-local with a TypeScript-private constructor, but an issued object's JavaScript prototype exposes its constructor. `cellOf()` trusts the private field on any constructed instance. `unsubscribeNode()` checks `cell.owner === this` and `active`, then `#detach()` rewrites the owner's linked-list head/tail from that cell's links. There is no constructor runtime mint in the reviewed source.

An adversarial `Reflect.construct(Object.getPrototypeOf(realSubscription).constructor, [craftedCell])` can therefore potentially supply a crafted cell whose `owner` is the genuine surface, `active` is true, and previous/next are null. If accepted, detachment can alter genuine list ownership despite no subscription admission. This is a prospective source-level mutation path, not a demonstrated successful runtime attack or release claim.

The UI executor is asked to reproduce rejection before mutation, preserving the real subscription, its reader ownership and exact terminal-close behavior. A module-private runtime mint at admission is a suitable narrow repair; a TypeScript assertion or class privacy annotation alone is not. The same review should check the exported private Surface patch constructor before relying on its privacy as authority, while retaining exact owner/source/patch checks at publication.

