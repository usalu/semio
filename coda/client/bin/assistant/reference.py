# #region 🔌Adapters
import os

from dotenv import load_dotenv
from google import genai

from semio.client.lib.py.main import *
# #endregion 🔌Adapters

# Getting files
dir_path = os.path.dirname(os.path.abspath(__file__))
input_path = "input/"
output_path = "output/"

data_file = "test-case-cleaned.json"
source_schema_file = "schema.json"

data_path = os.path.join(dir_path, input_path, data_file)
source_schema_path = os.path.join(dir_path, input_path, source_schema_file)

prompt_path = os.path.join(dir_path, "prompt.txt")

# Uploading files
load_dotenv()
api_key = os.environ.get("API_KEY")
client = genai.Client(api_key=api_key)

data_llm = client.files.upload(
    file=data_path, config={"mime_type": "text/plain", "display_name": data_file}
)
source_schema_llm = client.files.upload(
    file=source_schema_path,
    config={"mime_type": "text/plain", "display_name": source_schema_file},
)

# Getting prompt template
with open(prompt_path) as file:
    prompt_template = file.read()

# Iterating over rules
rules = get_rules()
for key, value in rules.items():
    target_schema = value["schema"]
    if target_schema == "owl":
        ext = "ttl"
    elif target_schema == "ids":
        ext = "xml"

    response_file = f"{key}.{ext}"
    response_path = os.path.join(dir_path, output_path, response_file)

    target_schema_path = get_rule(key)
    target_schema_file = f"{key}.txt"

    prompt = str(prompt_template).replace("[data]", data_file)
    prompt = prompt.replace("[source_schema]", source_schema_file)
    prompt = prompt.replace("[target_schema]", target_schema_file)

    target_schema_llm = client.files.upload(
        file=target_schema_path,
        config={"mime_type": "text/plain", "display_name": target_schema_file},
    )

    response = client.models.generate_content(
        model="gemini-2.5-flash",
        contents=[target_schema_llm, data_llm, source_schema_llm, prompt],
    )

    response_text = str(response.text)

    if target_schema == "owl":
        response_text = response_text.replace("```turtle\n", "")
    elif target_schema == "ids":
        response_text = response_text.replace("```xml\n", "")
    response_text = response_text.replace("\n```", "")

    with open(response_path, "w", encoding="utf-8") as file:
        file.write(response_text)

    client.files.delete(name=target_schema_llm.name)

    if target_schema == "owl":
        check_model(response_path)

# ---

from owlapy.class_expression import OWLClass
from owlapy.owl_reasoner import SyncReasoner
from owlapy.static_funcs import stopJVM


class SyncReasonerJustifications(SyncReasoner):
    def create_justifications(
        self, owl_individuals: None, owl_class_expression: None, save: bool = False
    ):
        """
        Generate multiple justifications for why the given individual(s) are inferred to be instances of the specified class.

        Args:
            owl_individuals (Set[OWLNamedIndividual]): Set of individuals to explain.
            owl_class_expression (OWLClassExpression): Class expression to justify.
            save (bool): If True, saves all justifications in a new ontology as axioms.

        Returns:
            List[Set[OWLAxiom]]: Each item is a justification (set of OWLAxioms).
        """
        if owl_individuals is None or owl_class_expression is None:
            raise ValueError(
                "Both owl_individuals and owl_class_expression are required."
            )

        from com.clarkparsia.owlapi.explanation import (
            BlackBoxExplanation,
            HSTExplanationGenerator,
            SatisfiabilityConverter,
        )
        from openllet.owlapi import PelletReasonerFactory

        j_class_expr = self.mapper.map_(owl_class_expression)
        j_ontology = self._owlapi_ontology
        j_reasoner = self._owlapi_reasoner
        j_data_factory = self._owlapi_manager.getOWLDataFactory()

        reasoner_factory = PelletReasonerFactory.getInstance()
        blackbox_exp = BlackBoxExplanation(j_ontology, reasoner_factory, j_reasoner)
        explanation_gen = HSTExplanationGenerator(blackbox_exp)
        converter = SatisfiabilityConverter(j_data_factory)

        justifications = {}

        for ind in owl_individuals:
            j_individual = self.mapper.map_(ind)
            class_assertion_axiom = j_data_factory.getOWLClassAssertionAxiom(
                j_class_expr, j_individual
            )
            unsat_class = converter.convert(class_assertion_axiom)

            j_explanations = explanation_gen.getExplanations(unsat_class)

            justification = []
            for j_expl in j_explanations:
                # py_axioms = {self.mapper.map_(ax) for ax in j_expl}
                py_axioms = []
                for ax in j_expl:
                    # print(ax)
                    # py_axiom = self.mapper.map_(ax)
                    py_axiom = str(ax)
                    py_axioms.append(py_axiom)
                py_axioms = list(set(py_axioms))
            justification.append(py_axioms)

        justifications[ind.iri.remainder] = justification

        stopJVM()

        return justifications


# ---

import json
import os
import shutil

import rdflib
from explain_owlapy import *
from owlready2 import *

dir_path = os.path.dirname(os.path.abspath(__file__))
rule_files = "rules/"
rule_base = "rules.json"
temp_dir = "temp/"
temp_rule = os.path.join(dir_path, temp_dir, "temp_rule.owl")
temp_data = os.path.join(dir_path, temp_dir, "temp_data.owl")
temp_nt = os.path.join(dir_path, temp_dir, "temp.nt")
temp_merged = os.path.join(dir_path, temp_dir, "temp_merged.owl")
temp_classified = os.path.join(dir_path, temp_dir, "temp_classified.owl")
log_file = "temp_log.txt"

rule_base_path = os.path.join(dir_path, rule_base)
with open(rule_base_path) as file:
    rules = json.load(file)


def get_rules():
    return rules


def get_rule(id: str):
    return os.path.join(dir_path, rule_files, rules[id]["file"])


def get_onto(response_path: str):
    g = rdflib.Graph()
    g.parse(response_path)
    g.serialize(destination=temp_nt, format="nt")

    onto = get_ontology(os.path.join("file://", temp_nt)).load()
    # for inst in onto.individuals():
    #    close_world(inst)
    # onto.save(temp_nt)
    return onto


def get_clauses(id: str):

    onto_path = get_rule(id)
    onto = get_onto(onto_path)

    classes = list(onto.classes())

    fail_class = onto["NotCompliant"]
    equivalent_to = fail_class.INDIRECT_equivalent_to[0]

    requirement = equivalent_to.is_a[1]
    requirement = requirement.Class
    # requirement = requirement.INDIRECT_equivalent_to[0]
    requirement = {
        "id": str(requirement).replace(f"{onto.name}.", ""),
        "description": requirement.label[0],
        "code": str(requirement.INDIRECT_equivalent_to[0]).replace(f"{onto.name}.", ""),
    }

    rationale = equivalent_to.is_a[0]
    rationale = rationale.INDIRECT_equivalent_to[0].Classes
    rationale = [
        {
            "id": str(x).replace(f"{onto.name}.", ""),
            "description": x.label[0],
            "code": str(x.INDIRECT_equivalent_to[0]).replace(f"{onto.name}.", ""),
        }
        for x in rationale
    ]

    res = {"id": id, "rationale": rationale, "requirement": requirement}

    onto.destroy(update_relation=True, update_is_a=True)

    return res


def copy_instances(source_path, target_path):

    source_onto = rdflib.Graph().parse(source_path)
    target_onto = rdflib.Graph().parse(target_path)
    merged = source_onto + target_onto
    merged.serialize(temp_merged, format="xml")
    return temp_merged


def check_model(data_path: str, rule_path: str):

    onto_path = copy_instances(data_path, rule_path)

    onto = get_onto(onto_path)

    try:
        # log_file_path = os.path.join(dir_path, log_file)
        # sys.stdout = open(log_file_path, 'w')
        with onto:
            sync_reasoner_pellet(
                infer_property_values=True, infer_data_property_values=True, debug=2
            )
            # sync_reasoner_pellet()
            onto.save(temp_classified)

        classes = list(onto.classes())
        for cl in classes:
            if cl.name == "NotCompliant":
                fail_class_iri = cl.iri
                break

        # sys.stdout.close()
        # with open(log_file_path) as file:
        #    log = file.read()

        reasoner = SyncReasonerJustifications(
            ontology=temp_classified, reasoner="Openllet"
        )
        # ontology_id = reasoner.ontology.get_ontology_id()
        # onto_iri = ontology_id._ontology_iri.str
        # fail_class_owlapy = OWLClass(f'{onto_iri}#NotCompliant')
        fail_class_owlapy = OWLClass(fail_class_iri)
        onto_iri = fail_class_owlapy.iri._namespace
        individuals_owlapy = reasoner.instances(fail_class_owlapy, direct=False)
        explanations_owlapy = reasoner.create_justifications(
            set(individuals_owlapy), fail_class_owlapy
        )
        for key, value in explanations_owlapy.items():
            explanation = []
            for x in value[0]:
                if "Assertion" in x:
                    # Formatting owlapy
                    x = x.replace(onto_iri, "")
                    x = x.replace("<", "")
                    x = x.replace(">", "")
                    x = x.replace("Object", "")
                    if "DataPropertyAssertion" in x:
                        x = x.replace("DataPropertyAssertion", "")
                        x = x[1:-1]
                        x = x.split(" ")
                        order = [1, 0, 2]
                        x = [x[i] for i in order]
                        x[2] = x[2].replace('"', "")
                        x[2] = x[2].replace("^^xsd:decimal", "")
                    if "ClassAssertion" in x:
                        x = x.replace("ClassAssertion", "")
                        x = x[1:-1]
                        if "AllValuesFrom" in x:
                            x = x.replace("AllValuesFrom", "")
                            x = x.rsplit(" ", 1)
                            x[0] = x[0][1:-1]
                            x = [x[1]] + x[0].split(" ", 1)
                            if "OneOf" in x[2]:
                                x[2] = x[2].replace("OneOf", "")
                                x[2] = x[2].replace("(", "")
                                x[2] = x[2].replace(")", "")
                                x[2] = x[2].split(" ")
                        if "ComplementOf" in x:
                            x = x.replace("ComplementOf", "")
                            x = x.split(" ")
                            x = [x[1], "Not", x[0]]
                    explanation.append(x)
                    # explanation = sorted(explanation)

            # Grouping by subject
            grouped = {}
            for triple in explanation:
                if grouped.get(triple[0]) == None:
                    grouped[triple[0]] = {triple[1]: triple[2]}
                else:
                    grouped[triple[0]][triple[1]] = triple[2]
            explanation = grouped

            explanations_owlapy[key] = explanation

        if len(explanations_owlapy) > 0:
            res = {"valid": False, "explanation": explanations_owlapy}
        else:
            res = {"valid": True}
        onto.destroy(update_relation=True, update_is_a=True)
        return res

    except Exception as e:
        if type(e).__name__ == "OwlReadyInconsistentOntologyError":
            res = {"valid": False, "log": str(e)}
            onto.destroy(update_relation=True, update_is_a=True)
            return res
        else:
            onto.destroy(update_relation=True, update_is_a=True)
            return str(e)


# data_path = os.path.join(os.path.abspath(os.path.join(os.getcwd(), os.pardir)), 'test/', 'test_v1.rdf')
# rule_path = os.path.join(os.path.abspath(os.path.join(os.getcwd(), os.pardir)), 'server/', 'rules/', 'OWL_test_1.owl')
# res = check_model(data_path, rule_path)
# with open('test.json', 'w') as file:
#    json.dump(res, file)

# ---

import uvicorn
from fastapi import FastAPI, File, UploadFile
from fastapi.responses import FileResponse

from semio.client.lib.py.main import *

app = FastAPI()


@app.get("/api/rule")
async def api_get_rules():
    return get_rules()


@app.get("/api/rule/{id}")
async def api_get_rule(id: str):
    return FileResponse(get_rule(id))


@app.get("/api/rule/{id}/clauses")
async def api_get_clauses(id: str):
    return get_clauses(id)


@app.post("/api/check_model")
async def api_check_model(
    data_file: UploadFile = File(...), rule_file: UploadFile = File(...)
):
    with open(temp_data, "wb") as f:
        shutil.copyfileobj(data_file.file, f)

    with open(temp_rule, "wb") as f:
        shutil.copyfileobj(rule_file.file, f)

    return check_model(temp_data, temp_rule)


if __name__ == "__main__":
    uvicorn.run(
        app,
        host="0.0.0.0" if os.environ.get("DEVCONTAINER") == "true" else "127.0.0.1",
        port=8000,
    )
