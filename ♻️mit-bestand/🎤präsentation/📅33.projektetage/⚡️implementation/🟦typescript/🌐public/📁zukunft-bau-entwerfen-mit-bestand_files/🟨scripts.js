/*! For license information please see 🟨scripts.js.LICENSE.txt */
(() => {
  "use strict";
  var e,
    t = {
      939(e, t, n) {
        var r = n(568),
          a = n(893),
          i = (n(972), n(534)),
          o = n.n(i);
        function l(e) {
          return (
            (l =
              "function" == typeof Symbol && "symbol" == typeof Symbol.iterator
                ? function (e) {
                    return typeof e;
                  }
                : function (e) {
                    return e && "function" == typeof Symbol && e.constructor === Symbol && e !== Symbol.prototype ? "symbol" : typeof e;
                  }),
            l(e)
          );
        }
        function s(e, t) {
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((r.enumerable = r.enumerable || !1), (r.configurable = !0), "value" in r && (r.writable = !0), Object.defineProperty(e, c(r.key), r));
          }
        }
        function c(e) {
          var t = (function (e) {
            if ("object" != l(e) || !e) return e;
            var t = e[Symbol.toPrimitive];
            if (void 0 !== t) {
              var n = t.call(e, "string");
              if ("object" != l(n)) return n;
              throw new TypeError("@@toPrimitive must return a primitive value.");
            }
            return String(e);
          })(e);
          return "symbol" == l(t) ? t : t + "";
        }
        var u = (function () {
          return (
            (e = function e() {
              var t = this;
              (!(function (e, t) {
                if (!(e instanceof t)) throw new TypeError("Cannot call a class as a function");
              })(this, e),
                r("body").delegate("a.solr-ajaxified", "click", function (e) {
                  return t.handleClickOnAjaxifiedUri(e);
                }));
            }),
            (t = [
              {
                key: "handleClickOnAjaxifiedUri",
                value: function (e) {
                  var t = r(e.target).closest("a"),
                    n = new URL(t.attr("href"), window.location.origin);
                  return ((window.location = n.href), !1);
                },
              },
              {
                key: "scrollToTopOfElement",
                value: function (e, t) {
                  r("html, body").animate({ scrollTop: e.offset().top - t + "px" }, "slow");
                },
              },
              {
                key: "setAjaxType",
                value: function (e) {
                  this.ajaxType = e;
                },
              },
            ]) && s(e.prototype, t),
            Object.defineProperty(e, "prototype", { writable: !1 }),
            e
          );
          var e, t;
        })();
        function f(e) {
          return (
            (f =
              "function" == typeof Symbol && "symbol" == typeof Symbol.iterator
                ? function (e) {
                    return typeof e;
                  }
                : function (e) {
                    return e && "function" == typeof Symbol && e.constructor === Symbol && e !== Symbol.prototype ? "symbol" : typeof e;
                  }),
            f(e)
          );
        }
        function d(e, t) {
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((r.enumerable = r.enumerable || !1), (r.configurable = !0), "value" in r && (r.writable = !0), Object.defineProperty(e, p(r.key), r));
          }
        }
        function p(e) {
          var t = (function (e) {
            if ("object" != f(e) || !e) return e;
            var t = e[Symbol.toPrimitive];
            if (void 0 !== t) {
              var n = t.call(e, "string");
              if ("object" != f(n)) return n;
              throw new TypeError("@@toPrimitive must return a primitive value.");
            }
            return String(e);
          })(e);
          return "symbol" == f(t) ? t : t + "";
        }
        n(13);
        var v = (function () {
          return (
            (e = function e() {
              (!(function (e, t) {
                if (!(e instanceof t)) throw new TypeError("Cannot call a class as a function");
              })(this, e),
                this.initToggle(),
                this.initFilter());
            }),
            (t = [
              {
                key: "reinit",
                value: function () {
                  (this.initToggle(), this.initFilter());
                },
              },
              {
                key: "initToggle",
                value: function () {
                  (r(".tx-solr-facet-hidden").hide(),
                    r("a.tx-solr-facet-show-all").click(function () {
                      return (
                        0 === r(this).parent().siblings(".tx-solr-facet-hidden:visible").length
                          ? (r(this).parent().siblings(".tx-solr-facet-hidden").show(), r(this).text(r(this).data("label-less")))
                          : (r(this).parent().siblings(".tx-solr-facet-hidden").hide(), r(this).text(r(this).data("label-more"))),
                        !1
                      );
                    }));
                },
              },
              {
                key: "initFilter",
                value: function () {
                  r(".facet-filter-box")
                    .closest(".facet")
                    .each(function () {
                      var e = r(this).find(".facet-filter-box"),
                        t = r(this).find(".facet-filter-item");
                      e.on("keyup", function () {
                        var n = e.val().toLowerCase();
                        t.each(function () {
                          var e = r(this);
                          e.toggle(e.text().toLowerCase().indexOf(n) > -1);
                        });
                      });
                    });
                },
              },
            ]) && d(e.prototype, t),
            Object.defineProperty(e, "prototype", { writable: !1 }),
            e
          );
          var e, t;
        })();
        function g(e) {
          return (
            (g =
              "function" == typeof Symbol && "symbol" == typeof Symbol.iterator
                ? function (e) {
                    return typeof e;
                  }
                : function (e) {
                    return e && "function" == typeof Symbol && e.constructor === Symbol && e !== Symbol.prototype ? "symbol" : typeof e;
                  }),
            g(e)
          );
        }
        function y(e, t) {
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((r.enumerable = r.enumerable || !1), (r.configurable = !0), "value" in r && (r.writable = !0), Object.defineProperty(e, h(r.key), r));
          }
        }
        function h(e) {
          var t = (function (e) {
            if ("object" != g(e) || !e) return e;
            var t = e[Symbol.toPrimitive];
            if (void 0 !== t) {
              var n = t.call(e, "string");
              if ("object" != g(n)) return n;
              throw new TypeError("@@toPrimitive must return a primitive value.");
            }
            return String(e);
          })(e);
          return "symbol" == g(t) ? t : t + "";
        }
        var m = (function () {
          return (
            (e = function e() {
              (!(function (e, t) {
                if (!(e instanceof t)) throw new TypeError("Cannot call a class as a function");
              })(this, e),
                this.reinit());
            }),
            (t = [
              {
                key: "reinit",
                value: function () {
                  r("form[data-suggest]").each(function () {
                    var e,
                      t = r(this),
                      n = t.find(".tx-solr-suggest");
                    ((e = t.find(".tx-solr-autocomplete").length > 0 ? t.find(".tx-solr-autocomplete") : r("body")),
                      r.ajaxSetup({ jsonp: "tx_solr[callback]" }),
                      0 === n.length && (n = t),
                      n.css("position", "relative"),
                      t.on("submit", function (e) {
                        "" === t.find(".tx-solr-suggest").val() && (e.preventDefault(), t.find(".tx-solr-suggest").focus());
                      }),
                      t
                        .find(".tx-solr-suggest")
                        .devbridgeAutocomplete({
                          serviceUrl: t.data("suggest"),
                          dataType: "jsonp",
                          paramName: "tx_solr[queryString]",
                          groupBy: "category",
                          maxHeight: 1e3,
                          appendTo: e,
                          autoSelectFirst: !1,
                          triggerSelectOnValidInput: !1,
                          width: 0.66 * n.outerWidth(),
                          onSelect: function (e) {
                            e.data.link ? (0 === e.data.link.indexOf("https://www.youtube.com") ? openVideoOverlay(e.data.link) : (location.href = e.data.link)) : t.trigger("submit");
                          },
                          transformResult: function (e) {
                            if (!e.suggestions) return { suggestions: [] };
                            var n,
                              a = {
                                suggestions: r.map(e.suggestions, function (e, t) {
                                  return (n || (n = t), { value: t, data: { category: "suggestion", count: e } });
                                }),
                              };
                            return (
                              r.each(e.documents, function (e, r) {
                                var i = r;
                                ((i.category = t.data("suggest-header") ? t.data("suggest-header") : "Top results"),
                                  i.group && (i.category = t.data("suggest-header-" + i.group) ? t.data("suggest-header-" + i.group) : i.group),
                                  a.suggestions.push({ value: n, data: i }));
                              }),
                              a
                            );
                          },
                          beforeRender: function (e) {
                            (e.find(".autocomplete-group:first").remove(), e.addClass("tx-solr-autosuggest"), n.parent().addClass("autocomplete-active").fadeIn());
                          },
                          formatResult: function (e, t) {
                            if (!t) return e.value;
                            var n = "(" + r.Autocomplete.utils.escapeRegExChars(t.trim()) + ")";
                            if ("suggestion" === e.data.category)
                              return e.value
                                .replace(new RegExp(n, "gi"), "<strong>$1</strong>")
                                .replace(/&/g, "&amp;")
                                .replace(/</g, "&lt;")
                                .replace(/>/g, "&gt;")
                                .replace(/"/g, "&quot;")
                                .replace(/&lt;(\/?strong)&gt;/g, "<$1>");
                            var a = e.data.title
                              .replace(new RegExp(n, "gi"), "<em>$1</em>")
                              .replace(/&/g, "&amp;")
                              .replace(/</g, "&lt;")
                              .replace(/>/g, "&gt;")
                              .replace(/"/g, "&quot;")
                              .replace(/&lt;(\/?em)&gt;/g, "<$1>");
                            return (
                              '<div class="' +
                              e.data.type +
                              '">' +
                              (e.data.previewImage ? "<figure " + (e.data.hasVideo ? 'class="hasVideo"' : "") + '><img src="' + e.data.previewImage + '" /></figure>' : "") +
                              '<a href="' +
                              e.data.link +
                              '" class="internal-link">' +
                              a +
                              "</a></div>"
                            );
                          },
                        })
                        .on("blur", function () {
                          n.parent().removeClass("autocomplete-active");
                          var e = r(this);
                          setTimeout(function () {
                            e.devbridgeAutocomplete("hide");
                          }, 200);
                        }));
                  });
                },
              },
            ]),
            t && y(e.prototype, t),
            Object.defineProperty(e, "prototype", { writable: !1 }),
            e
          );
          var e, t;
        })();
        function b(e) {
          return (
            (b =
              "function" == typeof Symbol && "symbol" == typeof Symbol.iterator
                ? function (e) {
                    return typeof e;
                  }
                : function (e) {
                    return e && "function" == typeof Symbol && e.constructor === Symbol && e !== Symbol.prototype ? "symbol" : typeof e;
                  }),
            b(e)
          );
        }
        function w(e, t) {
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((r.enumerable = r.enumerable || !1), (r.configurable = !0), "value" in r && (r.writable = !0), Object.defineProperty(e, _(r.key), r));
          }
        }
        function _(e) {
          var t = (function (e) {
            if ("object" != b(e) || !e) return e;
            var t = e[Symbol.toPrimitive];
            if (void 0 !== t) {
              var n = t.call(e, "string");
              if ("object" != b(n)) return n;
              throw new TypeError("@@toPrimitive must return a primitive value.");
            }
            return String(e);
          })(e);
          return "symbol" == b(t) ? t : t + "";
        }
        n(204);
        var k = (function () {
          return (
            (e = function e(t) {
              (!(function (e, t) {
                if (!(e instanceof t)) throw new TypeError("Cannot call a class as a function");
              })(this, e),
                (this.container = r(t)),
                (this.timers = null),
                this.reinit());
            }),
            (t = [
              {
                key: "reinit",
                value: function () {
                  var e = this,
                    t = (this.container.data("facet-name"), this.container.data("facet-url")),
                    n = this.container.find('[data-provide="slider"]');
                  (n.on("slide", function (n) {
                    var r = n.value[0],
                      a = n.value[1];
                    (isNaN(r) && (r = 0), isNaN(a) && (a = 0));
                    var i = t.replace("___FROM___", r.toString());
                    ((i = i.replace("___TO___", a.toString())), e.load(i));
                  }),
                    this.container.on("shown.bs.collapse", function () {
                      n.slider("relayout");
                    }));
                },
              },
              {
                key: "load",
                value: function (e) {
                  (clearTimeout(this.timers),
                    (this.timers = setTimeout(function () {
                      window.location.href = e;
                    }, 1500)));
                },
              },
            ]) && w(e.prototype, t),
            Object.defineProperty(e, "prototype", { writable: !1 }),
            e
          );
          var e, t;
        })();
        function S(e) {
          return (
            (S =
              "function" == typeof Symbol && "symbol" == typeof Symbol.iterator
                ? function (e) {
                    return typeof e;
                  }
                : function (e) {
                    return e && "function" == typeof Symbol && e.constructor === Symbol && e !== Symbol.prototype ? "symbol" : typeof e;
                  }),
            S(e)
          );
        }
        function x(e, t) {
          for (var n = 0; n < t.length; n++) {
            var r = t[n];
            ((r.enumerable = r.enumerable || !1), (r.configurable = !0), "value" in r && (r.writable = !0), Object.defineProperty(e, O(r.key), r));
          }
        }
        function O(e) {
          var t = (function (e) {
            if ("object" != S(e) || !e) return e;
            var t = e[Symbol.toPrimitive];
            if (void 0 !== t) {
              var n = t.call(e, "string");
              if ("object" != S(n)) return n;
              throw new TypeError("@@toPrimitive must return a primitive value.");
            }
            return String(e);
          })(e);
          return "symbol" == S(t) ? t : t + "";
        }
        var C = (function () {
          return (
            (e = function e(t) {
              (!(function (e, t) {
                if (!(e instanceof t)) throw new TypeError("Cannot call a class as a function");
              })(this, e),
                (this.container = r(t)),
                this.reinit());
            }),
            (t = [
              {
                key: "reinit",
                value: function () {
                  var e = this;
                  ((this.facetName = this.container.data("facetName")), (this.facetUrl = this.container.data("facetUrl")));
                  var t = this.container.find("input[type=date].startRange"),
                    n = this.container.find("input[type=date].endRange");
                  (t.off("change").change(function () {
                    e.solrRequest(e.facetName);
                  }),
                    n.off("change").change(function () {
                      e.solrRequest(e.facetName);
                    }));
                  var r = this.container.find("input[type=number].startRange");
                  if (r.attr("value")) {
                    var a = new Date(r.attr("value")).getFullYear();
                    (r.val(a), r.attr("value", null));
                  }
                  var i = this.container.find("input[type=number].endRange");
                  if (i.attr("value")) {
                    var o = new Date(i.attr("value")).getFullYear();
                    (i.val(o), i.attr("value", null));
                  }
                  (r.off("change").change(function () {
                    e.solrRequestYear(e.facetName);
                  }),
                    i.off("change").change(function () {
                      e.solrRequestYear(e.facetName);
                    }));
                },
              },
              {
                key: "solrRequest",
                value: function (e) {
                  var t = this.container.find("#start_date_" + e),
                    n = this.container.find("#end_date_" + e);
                  if ("" !== t.val() && "" !== n.val()) {
                    var r = this.container.find("#" + e + "_url").val(),
                      a = t.get(0).valueAsDate,
                      i = n.get(0).valueAsDate;
                    ((r = (r = r.replace(encodeURI("___FROM___"), this.formatDateForSolr(a))).replace(encodeURI("___TO___"), this.formatDateForSolr(i))), (window.location.href = r));
                  }
                },
              },
              {
                key: "solrRequestYear",
                value: function (e) {
                  var t = this.container.find("#start_date_" + e),
                    n = this.container.find("#end_date_" + e);
                  if ("" !== t.val() && "" !== n.val()) {
                    var r = this.container.find("#" + e + "_url").val(),
                      a = new Date(t.val(), 0, 1),
                      i = new Date(n.val(), 11, 31);
                    ((r = (r = r.replace(encodeURI("___FROM___"), this.formatDateForSolr(a))).replace(encodeURI("___TO___"), this.formatDateForSolr(i))), (window.location.href = r));
                  }
                },
              },
              {
                key: "formatDateForSolr",
                value: function (e) {
                  var t = e.getDate(),
                    n = e.getMonth() + 1;
                  return "" + e.getFullYear() + (n < 10 ? "0" + n : n) + (t < 10 ? "0" + t : t) + "0000";
                },
              },
            ]) && x(e.prototype, t),
            Object.defineProperty(e, "prototype", { writable: !1 }),
            e
          );
          var e, t;
        })();
        (o().setJQuery(r), a("flickity", o(), r));
        var j = document.getElementById("navbar"),
          R = j.offsetTop;
        ((window.openPic = function (e, t, n) {
          var r = window.open(e, t, n);
          r && r.focus();
        }),
          r(document).ready(function () {
            var e, t, n;
            ((e = r("#main-navbar")).find("[data-uid]").hover(function (t) {
              var n = r(t.target);
              if (
                (e.find("[data-uid]").removeClass("active"),
                e.find("[data-uid=" + n.data("uid") + "]").addClass("active"),
                e.find("[data-parent]").attr("hidden", "hidden"),
                e.find("[data-parent=" + n.data("uid") + "]").removeAttr("hidden"),
                n.data("parent"))
              ) {
                var a = e.find("[data-uid=" + n.data("parent") + "]");
                (a.removeAttr("hidden").addClass("active"),
                  e.find("[data-parent=" + n.data("parent") + "]").removeAttr("hidden"),
                  a.data("parent") &&
                    (e
                      .find("[data-uid=" + a.data("parent") + "]")
                      .removeAttr("hidden")
                      .addClass("active"),
                    e.find("[data-parent=" + a.data("parent") + "]").removeAttr("hidden")));
              }
            }),
              r(".sidebars .sidebar").each(function () {
                var e = r(this).attr("id");
                r("#" + e + "-link").click(function () {
                  var t = r("#" + e),
                    n = t.hasClass("active");
                  (r(".sidebars .sidebar").removeClass("active"), n || t.addClass("active"));
                });
              }),
              r(".carousel").each(function () {
                var e = r(this);
                if ("carousel-home-4139" !== e.attr("id")) {
                  var t = o().data(e[0]);
                  t && t.destroy();
                  var n = e.data("flickity");
                  (e.data("flickity", null),
                    e.attr("data-flickity", null),
                    n || (n = {}),
                    n.groupCells &&
                      (matchMedia("screen and (max-width: 576px)").matches || matchMedia("screen and (max-width: 768px)").matches
                        ? ((n.groupCells = !0), (n.contain = !1))
                        : matchMedia("screen and (max-width: 992px)").matches && ((n.groupCells = 2), (n.contain = !1))),
                    e.flickity(n),
                    e.find("img").each(function () {
                      r(this).on("load", function () {
                        e.flickity("resize");
                      });
                    }),
                    setTimeout(function () {
                      e.flickity("resize");
                    }, 100));
                }
              }),
              r(".search-toggle").click(function (e) {
                r("#search-bar").addClass("active");
              }),
              r(".search-close").click(function (e) {
                r("#search-bar").removeClass("active");
              }),
              new u().setAjaxType(7383),
              (t = new v()),
              r("body").on("tx_solr_updated", function () {
                t.reinit();
              }),
              (n = new m()),
              r("body").on("tx_solr_updated", function () {
                n.reinit();
              }),
              r(".facet-type-numericRange [data-facet-name]").each(function () {
                var e = new k(r(this));
                r("body").on("tx_solr_updated", function () {
                  e.reinit();
                });
              }),
              r(".facet-type-dateRange [data-facet-name]").each(function () {
                var e = new C(r(this));
                r("body").on("tx_solr_updated", function () {
                  e.reinit();
                });
              }),
              r("#filterDropdown").click(function () {
                (r("#tx-solr-faceting").toggle(), r("#overlay").toggle(), r("#filterResponsive").toggle(), r("#filterResponsiveClose").toggle(), r("#search-filter-responsive").toggle());
              }),
              r("#filterResponsiveClose").click(function () {
                (r("#tx-solr-faceting").toggle(), r("#overlay").toggle(), r("#filterResponsive").toggle(), r("#filterResponsiveClose").toggle(), r("#search-filter-responsive").toggle());
              }),
              r(".navbar-toggler").click(function () {
                (r("#main-navbar-responsive").toggle("show"), r(".overlayResponsive").toggle());
              }),
              r(".closing").click(function () {
                (r(".responsive-menu").removeClass("show"), r(".overlayResponsive").css("display", "none"));
              }),
              r(".search-toggle").click(function () {
                (r(".responsive-menu").css("display", "none"), r(".overlayResponsive").css("display", "none"));
              }),
              r(".background-trigger").click(function () {
                (r(this).children().hasClass("collapsed") ? r(this).addClass("background-yellow") : r(this).removeClass("background-yellow"),
                  r(this).parent().prev().hasClass("background-yellow") && r(this).parent().prev().removeClass("background-yellow"));
              }));
          }),
          r(window).scroll(function () {
            window.pageYOffset >= R ? j.classList.add("sticky") : j.classList.remove("sticky");
          }));
      },
    },
    n = {};
  function r(e) {
    var a = n[e];
    if (void 0 !== a) return a.exports;
    var i = (n[e] = { exports: {} });
    return (t[e].call(i.exports, i, i.exports, r), i.exports);
  }
  ((r.m = t),
    (e = []),
    (r.O = (t, n, a, i) => {
      if (!n) {
        var o = 1 / 0;
        for (u = 0; u < e.length; u++) {
          for (var [n, a, i] = e[u], l = !0, s = 0; s < n.length; s++) (!1 & i || o >= i) && Object.keys(r.O).every((e) => r.O[e](n[s])) ? n.splice(s--, 1) : ((l = !1), i < o && (o = i));
          if (l) {
            e.splice(u--, 1);
            var c = a();
            void 0 !== c && (t = c);
          }
        }
        return t;
      }
      i = i || 0;
      for (var u = e.length; u > 0 && e[u - 1][2] > i; u--) e[u] = e[u - 1];
      e[u] = [n, a, i];
    }),
    (r.n = (e) => {
      var t = e && e.__esModule ? () => e.default : () => e;
      return (r.d(t, { a: t }), t);
    }),
    (r.d = (e, t) => {
      for (var n in t) r.o(t, n) && !r.o(e, n) && Object.defineProperty(e, n, { enumerable: !0, get: t[n] });
    }),
    (r.o = (e, t) => Object.prototype.hasOwnProperty.call(e, t)),
    (r.r = (e) => {
      ("undefined" != typeof Symbol && Symbol.toStringTag && Object.defineProperty(e, Symbol.toStringTag, { value: "Module" }), Object.defineProperty(e, "__esModule", { value: !0 }));
    }),
    (() => {
      var e = { 385: 0 };
      r.O.j = (t) => 0 === e[t];
      var t = (t, n) => {
          var a,
            i,
            [o, l, s] = n,
            c = 0;
          if (o.some((t) => 0 !== e[t])) {
            for (a in l) r.o(l, a) && (r.m[a] = l[a]);
            if (s) var u = s(r);
          }
          for (t && t(n); c < o.length; c++) ((i = o[c]), r.o(e, i) && e[i] && e[i][0](), (e[i] = 0));
          return r.O(u);
        },
        n = (self.webpackChunk = self.webpackChunk || []);
      (n.forEach(t.bind(null, 0)), (n.push = t.bind(null, n.push.bind(n))));
    })());
  var a = r.O(void 0, [84, 634], () => r(939));
  a = r.O(a);
})();
