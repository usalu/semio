import sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
a='''  issue() {
    if (this.#closed || this.#outstanding || this.#receiptRefusal || this.#producerRefusal !== void 0 || this.#nativeRefusal !== void 0)
      throw'''
b='''  issue() {
    if (this.#closed || this.#outstanding || this.#receiptRefusal || this.#producerRefusal !== void 0 || this.#nativeRefusal !== void 0)
      console.error("[DEBUG] source-not-ready", JSON.stringify({ closed: this.#closed, outstanding: this.#outstanding, receiptRefusal: this.#receiptRefusal, producerRefusal: this.#producerRefusal, nativeRefusal: this.#nativeRefusal }, (k, v) => typeof v === "bigint" ? v.toString() : v), new Error().stack);
    if (this.#closed || this.#outstanding || this.#receiptRefusal || this.#producerRefusal !== void 0 || this.#nativeRefusal !== void 0)
      throw'''
assert s.count(a)==1
s=s.replace(a,b)
c="      } catch (failure) {\n        try {\n          if (failure && failure.input"
d="      } catch (failure) {\n        console.error(\"[DEBUG] poll failure\", failure, JSON.stringify(failure, (k, v) => typeof v === \"bigint\" ? v.toString() : v));\n        try {\n          if (failure && failure.input"
assert s.count(c)==1
s=s.replace(c,d)
open(p,"w",encoding="utf-8").write(s)
